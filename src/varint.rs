use std::fmt::{Display, Formatter};
use std::io::Write;

#[derive(Debug)]
pub struct VarInt(i64);

impl Display for VarInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i64> for VarInt {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl Write for VarInt {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}

impl VarInt {
    pub fn get_val(&self) -> i64 {
        self.0
    }
}
