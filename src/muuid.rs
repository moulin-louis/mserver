use bincode::enc::Encoder;
use bincode::Encode;
use bincode::error::EncodeError;
use uuid::Uuid;

pub struct MUuid(Uuid);

impl Encode for MUuid {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        bincode::Encode::encode(&self.0.as_bytes(), encoder)
    }
}

impl From<Uuid> for MUuid {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}