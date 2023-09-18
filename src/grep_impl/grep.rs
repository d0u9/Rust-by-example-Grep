use crate::err::AppErr;

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn grep_from_file(file: &str, needle: &str) -> Result<Vec<String>, AppErr> {
    // Read file, and return error if `open()` failed
    let file = match File::open(file) {
        Ok(file) => file,
        Err(e) => return Err(AppErr { msg: e.to_string() }),
    };

    let mut matched_lines = Vec::new();
    let Ok(needle) = Regex::new(needle) else {
        return Err(AppErr { msg: "cannot create regex".to_string(), });
    };

    // Create a BufReader for our file
    let reader = BufReader::new(file);

    // Iterate over whole file line by line
    for line in reader.lines() {
        // read.lines() iterates over each line, and return Result<String, Error>
        let line = match line {
            Ok(line) => line,
            Err(e) => return Err(AppErr { msg: e.to_string() }),
        };

        if needle.captures(&line).is_some() {
            matched_lines.push(line);
        }
    }

    Ok(matched_lines)
}
