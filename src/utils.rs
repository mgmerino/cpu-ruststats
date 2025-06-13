// src/utils.rs
// Heavily commented Rust code because I'm an absolute beginner
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

const SPARK_CHARS: &[&str] = &["▁", "▂", "▃", "▄", "▅", "▆", "▇"];

// Reads the history file passed as a string path
// Returns a Vec<f64> with all values, or empty one if there
// was an error reading the file
pub fn read_history(path: &str) -> Vec<f64> {
    // first, check the path exists, otherwise,
    // return early the expected data type (empty vector)
    if !Path::new(path).exists() {
        return Vec::new();
    }
    // create a buffer to save the file content in memory
    let mut buf = String::new();
    // try to read the file
    if let Ok(mut f) = fs::File::open(path) {
        // save file contents in buffer
        let _ = f.read_to_string(&mut buf);
        // parse or return empty vector
        serde_json::from_str(&buf).unwrap_or_default()
    } else {
        // no file, return empty vector
        Vec::new()
    }
}

// Accepts path as string and history values as a dynamically-sized
// view into a contiguous sequence (i.e.: a slice, represented as &[f64]).
// https://doc.rust-lang.org/std/primitive.slice.html
// It fails silently if the file cannot be opened.
pub fn write_history(path: &str, hist: &[f64]) {
    // try to open the file using OpenOptions
    // https://doc.rust-lang.org/std/fs/struct.OpenOptions.html
    if let Ok(mut f) = OpenOptions::new()
        // create if does not exist
        .create(true)
        // open in write-mode
        .write(true)
        // destroy previous content
        .truncate(true)
        .open(path)
    {
        // parse file
        let data = serde_json::to_string(hist).unwrap();
        // write new buffer to file
        let _ = f.write_all(data.as_bytes());
    }
}

// Accepts a slice of f64 values and returns a sparkline
// string representation of the data.
pub fn make_sparkline(data: &[f64]) -> String {
    // first, check the if the data is not empty, otherwise,
    // return early the expected data type (string)
    if data.is_empty() {
        return String::new();
    }

    // Get min and max value in data:
    // iterate over data values,
    // clone the references to own values,
    // fold the min/max values from/to "infinity"/"negative infinity"
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    // Calculate range between min and max
    let range = if (max - min).abs() < std::f64::EPSILON {
        // avoid zero division
        1.0
    } else {
        max - min
    };

    // Create new vector for chars
    let mut chars: Vec<&str> = Vec::new();
    // Iterate over data values
    for &v in data.iter() {
        // Normalize value between 0 and 1
        let ratio = (v - min) / range;
        // Scale to index in SPARK_CHARS
        let scaled = ratio * ((SPARK_CHARS.len() - 1) as f64);
        // Round and convert to usize
        let idx = scaled.round() as usize;
        // Push the selected character into the buffer
        chars.push(SPARK_CHARS[idx]);
    }
    // Join all the chars in the buffer
    let spark = chars.join("");

    spark
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_read_history_empty_file() {
        let path = "/tmp/test_read_history_empty.json";
        let _ = fs::remove_file(path);
        let result = read_history(path);
        assert!(result.is_empty());
    }

    #[test]
    fn test_read_history_valid_data() {
        let path = "/tmp/test_read_history_valid.json";
        let test_data = vec![1.0, 2.0, 3.0];
        write_history(path, &test_data);
        let result = read_history(path);
        assert_eq!(result, test_data);
        // Clean up
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_write_history() {
        let path = "/tmp/test_write_history.json";
        let test_data = vec![1.0, 2.0, 3.0];
        write_history(path, &test_data);
        assert!(Path::new(path).exists());
        let content = fs::read_to_string(path).unwrap();
        let parsed: Vec<f64> = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed, test_data);
        // Clean up
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_make_sparkline_empty() {
        let data: Vec<f64> = vec![];
        let result = make_sparkline(&data);
        assert_eq!(result, "");
    }

    #[test]
    fn test_make_sparkline_single_value() {
        let data = vec![1.0];
        let result = make_sparkline(&data);
        assert_eq!(result, "▁");
    }

    #[test]
    fn test_make_sparkline_multiple_values() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = make_sparkline(&data);
        // Verify that the result has the correct length
        assert_eq!(result.chars().count(), 5);
        // Verify that the first character is the lowest and the last is the highest
        assert_eq!(result.chars().next().unwrap().to_string(), "▁");
        assert_eq!(result.chars().last().unwrap().to_string(), "▇");
    }
}
