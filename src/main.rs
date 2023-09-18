mod err;
mod grep_impl;
mod version;

use grep_impl::grep;
use version::*;

use std::env;

fn main() {
    version_manipulation().unwrap();

    let mut args = env::args();
    let _prog = args.next().unwrap();
    let Some(pattern) = args.next() else {
        println!("OPTION ERROR: missing PATTERN option");
        return;
    };
    let Some(file) = args.next() else {
        println!("OPTION ERROR: missing FILE option");
        return;
    };

    let matched_lines = match grep::grep_from_file(&file, &pattern) {
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
