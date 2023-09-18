use crate::err::AppErr;
use crate::grep_impl::find_greper::*;
use crate::grep_impl::interface::*;
use crate::grep_impl::regex_greper::*;
use std::env;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn grep_from_file(file: &str, needle: &str) -> Result<Vec<String>, AppErr> {
    let greper: Box<dyn Greper> = match env::var("BACKEND") {
        Ok(backend) => match backend.to_ascii_lowercase().as_ref() {
            "regex" => Box::new(RegexGreper::new(needle)?),
            _ => Box::new(FindGreper::new(needle)?),
        },
        Err(_) => Box::new(FindGreper::new(needle)?),
    };

    grep_from_file_by_greper(file, &*greper)
}

fn grep_from_file_by_greper(file: &str, greper: &dyn Greper) -> Result<Vec<String>, AppErr> {
    println!("==>> Using {}", greper.name());

    let file = File::open(file)?;

    let mut matched_lines = Vec::new();

    // Create a BufReader for our file
    let reader = BufReader::new(file);

    // Iterate over whole file line by line
    for line in reader.lines() {
        // read.lines() iterates over each line, and return Result<String, Error>
        let line = line?;

        if greper.grep(&line) {
            matched_lines.push(line);
        }
    }

    Ok(matched_lines)
}
