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

    let matched_lines = BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .filter(|line| greper.grep(line))
        .collect::<Vec<_>>();

    Ok(matched_lines)
}
