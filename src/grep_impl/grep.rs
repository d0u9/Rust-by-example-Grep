use crate::err::AppErr;
use crate::grep_impl::regex_greper::*;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn grep_from_file(file: &str, needle: &str) -> Result<Vec<String>, AppErr> {
    // Read file, and return error if `open()` failed
    let file = match File::open(file) {
        Ok(file) => file,
        Err(e) => return Err(AppErr { msg: e.to_string() }),
    };

    let mut matched_lines = Vec::new();
    let greper = RegexGreper::new(needle)?;

    // Create a BufReader for our file
    let reader = BufReader::new(file);

    // Iterate over whole file line by line
    for line in reader.lines() {
        // read.lines() iterates over each line, and return Result<String, Error>
        let line = match line {
            Ok(line) => line,
            Err(e) => return Err(AppErr { msg: e.to_string() }),
        };

        if greper.grep(&line) {
            matched_lines.push(line);
        }
    }

    Ok(matched_lines)
}
