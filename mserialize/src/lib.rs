use std::io::{Cursor, Write};

pub trait MSerialize {
    fn to_bytes_representation(&self) -> Box<[u8]>;
}

macro_rules! impl_for_primitives {
    ($($t:ty)*) => ($(
        impl MSerialize for $t {
            fn to_bytes_representation(&self) -> Box<[u8]> {
                Box::new(self.to_be_bytes())
            }
        }
    )*)
}

impl_for_primitives! { i32 u32 i64 u64 f32 f64 }

impl MSerialize for String {
    fn to_bytes_representation(&self) -> Box<[u8]> {
        let mut cursor = Cursor::new(Vec::with_capacity(self.len() + 4));
        leb128::write::signed(&mut cursor, self.len() as i64).unwrap();
        cursor.write_all(self.as_bytes()).unwrap();
        cursor.into_inner().into_boxed_slice()
    }
}