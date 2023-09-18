mod err;
mod grep_impl;
mod version;

use grep_impl::grep;
use version::*;

fn main() {
    version_manipulation().unwrap();

    let matched_lines = match grep::grep_from_file("/etc/passwd", "root") {
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
