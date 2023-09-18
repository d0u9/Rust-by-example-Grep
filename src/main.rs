use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct AppErr {
    msg: String,
}

fn main() {
    version_manipulation().unwrap();

    let matched_lines = match grep_from_file("/etc/passwd", "root") {
        Ok(lines) => lines,
        Err(e) => {
            println!("GREP ERROR: {}", e.msg);
            return;
        }
    };

    println!("==================== Below are matching lines =======================");
    for line in matched_lines.into_iter() {
        println!("{}", line);
    }
}

fn grep_from_file(file: &str, needle: &str) -> Result<Vec<String>, AppErr> {
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

fn version_manipulation() -> Result<(), AppErr> {
    let is_dev = true;
    let version = String::from("5.16");
    let suffix = "-dev";

    if version.find('.').is_none() {
        return Err(AppErr {
            msg: "invalid version number".to_string(),
        });
    }

    // Use ? to Propagate the error
    version_verify_and_compare(&version)?;

    print_version(version, suffix, is_dev);

    Ok(())
}

fn print_version(mut version: String, suffix: &str, is_dev: bool) {
    if is_dev {
        version += suffix;
    }
    println!("version: {}", version);
}

fn version_verify_and_compare(version: &str) -> Result<(), AppErr> {
    let version_f64 = match version.parse::<f64>() {
        Ok(v) => v,
        Err(e) => return Err(AppErr { msg: e.to_string() }),
    };

    if version_f64 > 5.0 {
        println!("version is greater than 5.0");
    }
    Ok(())
}
