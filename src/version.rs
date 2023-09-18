use crate::err::AppErr;

pub fn version_manipulation() -> Result<(), AppErr> {
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
