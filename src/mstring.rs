use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct MString(String);

impl MString {
    pub fn new() -> Self {
        Self(String::new())
    }
}

impl Display for MString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for MString {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<Vec<u8>> for MString {
    fn from(value: Vec<u8>) -> Self {
        Self(String::from_utf8(value).unwrap())
    }
}
