use std::fmt::{Display, Formatter};
use std::io::{Cursor, Write};

use bincode::Encode;

use mserialize::MSerialize;

#[derive(Debug, Clone, Encode)]
pub struct MString(pub String);

impl MString {
    pub fn new() -> Self {
        Self(String::new())
    }
}

impl MSerialize for MString {
    fn to_bytes_representation(&self) -> Box<[u8]> {
        let mut cursor = Cursor::new(Vec::with_capacity(self.0.len() + 4));
        leb128::write::signed(&mut cursor, self.0.len() as i64).unwrap();
        cursor.write_all(self.0.as_bytes()).unwrap();
        cursor.into_inner().into_boxed_slice()
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

impl From<String> for MString {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<Vec<u8>> for MString {
    fn from(value: Vec<u8>) -> Self {
        Self(String::from_utf8(value).unwrap())
    }
}
