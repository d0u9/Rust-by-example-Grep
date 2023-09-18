use crate::err::AppErr;
use crate::grep_impl::interface::*;

pub struct FindGreper {
    needle: String,
}

impl FindGreper {
    pub fn new(needle: &str) -> Result<Self, AppErr> {
        Ok(FindGreper {
            needle: needle.to_owned(),
        })
    }
}

impl Greper for FindGreper {
    fn grep(&self, line: &str) -> bool {
        line.contains(&self.needle)
    }

    fn name(&self) -> &str {
        "Find Greper: Grep Backed based on Rust standard library's string finding function"
    }
}
