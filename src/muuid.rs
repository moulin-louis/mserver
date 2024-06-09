use bincode::Encode;
use serde::Serialize;
use uuid::{Bytes, Uuid};

use mserialize::MSerialize;

#[derive(Serialize, Debug, Copy, Clone, Encode)]
pub struct Muuid(Bytes);

impl MSerialize for Muuid {
    fn to_bytes_representation(&self) -> Box<[u8]> {
        Box::new(self.0.clone())
    }
}

impl From<Bytes> for Muuid {
    fn from(value: Bytes) -> Self {
        Self(value)
    }
}

impl From<Uuid> for Muuid {
    fn from(value: Uuid) -> Self {
        Self(value.into_bytes())
    }
}