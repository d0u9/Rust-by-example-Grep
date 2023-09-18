fn main() {
    version_manipulation();
}

fn version_manipulation() {
    let is_dev = true;
    let version = String::from("5.16");
    let suffix = "-dev";

    if version.find('.').is_none() {
        println!("invalid version number");
        return;
    }

    version_verify_and_compare(&version).unwrap();

    print_version(version, suffix, is_dev);
}

fn print_version(mut version: String, suffix: &str, is_dev: bool) {
    if is_dev {
        version += suffix;
    }
    println!("version: {}", version);
}

fn version_verify_and_compare(version: &str) -> Result<(), std::num::ParseFloatError> {
    let version_f64 = match version.parse::<f64>() {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    if version_f64 > 5.0 {
        println!("version is greater than 5.0");
    }
    Ok(())
}
