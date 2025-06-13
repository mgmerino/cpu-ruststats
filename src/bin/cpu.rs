// src/bin/cpu.rs

use clap::Command;
use cpu_ruststats::utils;
use std::fs::{self};
use std::io::{BufRead, BufReader};
use std::time::Duration; // Import utils module

const PROC_STAT_PATH: &str = "/proc/stat";
const HISTORY_PATH: &str = "/tmp/cpu_usage_history.json";
const DEFAULT_POINTS: usize = 20;

fn main() {
    let matches = Command::new("cpu")
        .about("CPU usage monitor for i3blocks/i3status using /proc/stat")
        .arg(
            clap::Arg::new("warning")
                .short('w')
                .long("warning")
                .value_name("WARN")
                .help("Umbral de advertencia en porcentaje")
                .value_parser(clap::value_parser!(f64))
                .default_value("70.0"),
        )
        .arg(
            clap::Arg::new("critical")
                .short('c')
                .long("critical")
                .value_name("CRIT")
                .help("Umbral crítico en porcentaje")
                .value_parser(clap::value_parser!(f64))
                .default_value("90.0"),
        )
        .arg(
            clap::Arg::new("count")
                .short('n')
                .long("count")
                .value_name("N")
                .help("Longitud del sparkline")
                .value_parser(clap::value_parser!(usize))
                .default_value("20"),
        )
        .get_matches();

    // read command line arguments
    let warn = matches.get_one::<f64>("warning").cloned().unwrap_or(70.0);
    let crit = matches.get_one::<f64>("critical").cloned().unwrap_or(90.0);
    let max_len = matches
        .get_one::<usize>("count")
        .cloned()
        .unwrap_or(DEFAULT_POINTS);

    // read initial snapshot
    let (total0, idle0) = match read_proc_stat() {
        Some(data) => data,
        None => {
            eprintln!("Error al leer /proc/stat");
            return;
        }
    };

    // wait for a short period to get a second snapshot
    std::thread::sleep(Duration::from_millis(100));

    // read second snapshot
    let (total1, idle1) = match read_proc_stat() {
        Some(data) => data,
        None => {
            eprintln!("Error al leer /proc/stat");
            return;
        }
    };

    // compute deltas
    let delta_total = total1 - total0;
    let delta_idle = idle1 - idle0;

    // calculate CPU usage percentage
    let usage = if delta_total > 0 {
        100.0 * (delta_total - delta_idle) as f64 / delta_total as f64
    } else {
        0.0
    };

    // update history
    let mut history = utils::read_history(HISTORY_PATH);
    history.push(usage);

    // remover oldest entries if history exceeds max_len
    if history.len() > max_len {
        history.drain(0..history.len() - max_len);
    }

    // write updated history back to file
    utils::write_history(HISTORY_PATH, &history);

    // generate sparkline
    let spark = utils::make_sparkline(&history);

    // print output
    //println!("{}%", usage as u8);
    println!("{:.1}% {}", usage, spark);
    if usage >= crit {
        {
            println!("#FF0000");
            std::process::exit(33);
        }
    } else if usage >= warn {
        {
            println!("#FFFC00");
        }
    }
}

// reads the /proc/stat file and returns the total CPU time and idle time
fn read_proc_stat_with_path(path: &str) -> Option<(u64, u64)> {
    // Open the file
    if let Ok(f) = fs::File::open(path) {
        let reader = BufReader::new(f);
        for line_result in reader.lines() {
            let line = line_result.unwrap();
            if line.starts_with("cpu ") {
                let fields: Vec<&str> = line.split_whitespace().collect();
                if fields.len() < 5 {
                    return None; // Not enough fields
                }
                let user: u64 = fields[1].parse().unwrap_or(0);
                let nice: u64 = fields[2].parse().unwrap_or(0);
                let system: u64 = fields[3].parse().unwrap_or(0);
                let idle: u64 = fields[4].parse().unwrap_or(0);
                let total = user + nice + system + idle;
                return Some((total, idle));
            }
        }
    } else {
        return None;
    }
    None
}

fn read_proc_stat() -> Option<(u64, u64)> {
    read_proc_stat_with_path(PROC_STAT_PATH)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_read_proc_stat() {
        // Create a temporary file with mock data
        let temp_path = "/tmp/test_proc_stat";
        let mut file = fs::File::create(temp_path).unwrap();
        writeln!(file, "cpu  10 20 30 40").unwrap();
        file.sync_all().unwrap();

        // Test reading the file
        let result = read_proc_stat_with_path(temp_path);
        assert!(result.is_some());
        let (total, idle) = result.unwrap();
        assert_eq!(total, 100); // 10 + 20 + 30 + 40
        assert_eq!(idle, 40);

        // Clean up
        fs::remove_file(temp_path).unwrap();
    }

    #[test]
    fn test_read_proc_stat_empty() {
        // Create an empty file
        let temp_path = "/tmp/test_proc_stat_empty";
        fs::File::create(temp_path).unwrap();

        // Test reading the empty file
        let result = read_proc_stat_with_path(temp_path);
        assert!(result.is_none());

        // Clean up
        fs::remove_file(temp_path).unwrap();
    }
}
