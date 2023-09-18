pub trait Greper {
    fn grep(&self, line: &str) -> bool;
    fn name(&self) -> &str;
}
