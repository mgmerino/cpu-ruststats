// src/bin/temperature.rs

use clap::{Arg, Command};
use cpu_ruststats::utils::{make_sparkline, read_history, write_history};
use serde_json::Value;
use std::process::{Command as Cmd, exit};

const HISTORY_PATH: &str = "/tmp/temperature_history.json";

fn extract_temperatures(json: &serde_json::Value) -> Vec<f64> {
    let mut temps = Vec::new();
    if let Value::Object(chips) = json {
        for (_chip, data) in chips {
            if let Value::Object(entries) = data {
                for (_label, metrics) in entries {
                    if let Value::Object(fields) = metrics {
                        for (k, v) in fields {
                            if k.ends_with("_input") && k.starts_with("temp") {
                                if let Some(val) = v.as_f64() {
                                    temps.push(val);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    temps
}

fn main() {
    let matches = Command::new("temperature")
        .about("Shows sensors avg temperature with ASCII sparkline")
        .arg(
            Arg::new("warning")
                .short('w')
                .long("warning")
                .value_name("WARN")
                .help("Warning threshold")
                .value_parser(clap::value_parser!(f64))
                .default_value("70"),
        )
        .arg(
            Arg::new("critical")
                .short('c')
                .long("critical")
                .value_name("CRIT")
                .help("Critical threshold")
                .value_parser(clap::value_parser!(f64))
                .default_value("90"),
        )
        .arg(
            Arg::new("chip")
                .long("chip")
                .value_name("CHIP")
                .help("Sensor chip"),
        )
        .arg(
            Arg::new("count")
                .short('n')
                .long("count")
                .value_name("N")
                .help("Sparkline length")
                .value_parser(clap::value_parser!(usize))
                .default_value("5"),
        )
        .get_matches();

    let warn = *matches.get_one::<f64>("warning").unwrap();
    let crit = *matches.get_one::<f64>("critical").unwrap();
    let chip = matches.get_one::<String>("chip");
    let max_len = *matches.get_one::<usize>("count").unwrap();

    // Run `sensors -j`
    let mut cmd = Cmd::new("sensors");
    cmd.arg("-j");
    if let Some(ch) = chip {
        cmd.arg(ch);
    }
    let output = cmd.output().expect("`sensors` cmd failed");
    if !output.status.success() {
        eprintln!("Error when running `sensors`");
        exit(1);
    }

    let json: Value = serde_json::from_slice(&output.stdout).expect("Invalid JSON");
    let temps = extract_temperatures(&json);
    if temps.is_empty() {
        eprintln!("No temperature sensor available");
        exit(1);
    }

    let avg = temps.iter().copied().sum::<f64>() / (temps.len() as f64);

    let mut history = read_history(HISTORY_PATH);
    history.push(avg);
    if history.len() > max_len {
        history.drain(0..history.len() - max_len);
    }
    write_history(HISTORY_PATH, &history);

    let spark = make_sparkline(&history);

    let icon = match avg {
        x if x < 25.0 => "",
        x if x < 35.0 => "",
        x if x < 65.0 => "",
        x if x < 75.0 => "",
        _ => "",
    };

    //println!("{} {}°C", icon, avg as i32); // short_text
    println!("{} {:.1}°C {}", icon, avg, spark); // full_text + sparkline

    if avg >= crit {
        println!("#FF0000");
        exit(33);
    } else if avg >= warn {
        println!("#FFFC00");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_temperature() {
        // Mock the `sensors -j` command output
        let output = r#"{"coretemp-isa-0000":{"Core 0":{"temp1_input":50.0},"Core 1":{"temp2_input":55.0}}}"#;
        let result = serde_json::from_str::<serde_json::Value>(output).unwrap();
        let temps = extract_temperatures(&result);
        assert_eq!(temps.len(), 2);
        assert_eq!(temps[0], 50.0);
        assert_eq!(temps[1], 55.0);
    }

    #[test]
    fn test_read_temperature_no_data() {
        // Mock empty output
        let output = r#"{}"#;
        let result = serde_json::from_str::<serde_json::Value>(output).unwrap();
        let temps = extract_temperatures(&result);
        assert!(temps.is_empty());
    }
}
