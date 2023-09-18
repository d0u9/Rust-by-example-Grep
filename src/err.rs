#[derive(Debug)]
pub struct AppErr {
    pub(crate) msg: String,
}

impl From<std::io::Error> for AppErr {
    fn from(value: std::io::Error) -> Self {
        Self {
            msg: value.to_string(),
        }
    }
}
