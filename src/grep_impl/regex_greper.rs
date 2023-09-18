use crate::err::AppErr;

use regex::Regex;

pub struct RegexGreper {
    regex: Regex,
}

impl RegexGreper {
    pub fn new(needle: &str) -> Result<Self, AppErr> {
        let Ok(regex) = Regex::new(needle) else {
            return Err(AppErr { msg: "cannot create regex".to_string(), });
        };

        Ok(Self { regex })
    }

    pub fn grep(&self, line: &str) -> bool {
        self.regex.captures(line).is_some()
    }
}
