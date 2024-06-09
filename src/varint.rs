use std::fmt::{Display, Formatter};
use std::io::Write;

use bincode::enc::Encoder;
use bincode::Encode;
use bincode::error::EncodeError;
use serde::Serialize;

use mserialize::MSerialize;

#[derive(Debug, Serialize, Default)]
pub struct VarInt(pub i64);


impl VarInt {
    pub fn get_val(&self) -> i64 {
        self.0
    }
}

impl Encode for VarInt {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        bincode::Encode::encode(&self.0, encoder)
    }
}


impl MSerialize for VarInt {
    fn to_bytes_representation(&self) -> Box<[u8]> {
        let mut buff: Vec<u8> = Vec::with_capacity(8);
        leb128::write::signed(&mut buff, self.0).unwrap();
        buff.to_vec().into_boxed_slice()
    }
}

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

impl Into<usize> for VarInt {
    fn into(self) -> usize {
        self.0.clone() as usize
    }
}

impl Write for VarInt {
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}

