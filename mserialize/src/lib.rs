use std::io::{Cursor, Write};

pub trait MSerialize {
    fn to_bytes_representation(&self) -> Box<[u8]>;

    fn size_bytes(&self) -> usize {
        self.to_bytes_representation().len()
    }
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

impl_for_primitives! { i8 u8 i16 u16 i32 u32 i64 u64 f32 f64 }

impl MSerialize for bool {
    fn to_bytes_representation(&self) -> Box<[u8]> {
        (*self as u8).to_bytes_representation()
    }
}

impl MSerialize for String {
    fn to_bytes_representation(&self) -> Box<[u8]> {
        let mut cursor = Cursor::new(Vec::with_capacity(self.len() + 4));
        leb128::write::signed(&mut cursor, self.len() as i64).unwrap();
        cursor.write_all(self.as_bytes()).unwrap();
        cursor.into_inner().into_boxed_slice()
    }

    fn size_bytes(&self) -> usize {
        self.to_bytes_representation().len()
    }
}

impl<T> MSerialize for Vec<T>
where
        T: MSerialize,
{
    fn to_bytes_representation(&self) -> Box<[u8]> {
        let mut result: Vec<u8> = Vec::new();
        let all_bytes: Vec<Box<[u8]>> = self.iter().map(|x| x.to_bytes_representation()).collect();
        all_bytes.iter().for_each(|x| {
            result.append(&mut x.to_vec());
        });
        result.into_boxed_slice()
    }
}

impl<T> MSerialize for Option<T>
where
        T: MSerialize,
{
    fn to_bytes_representation(&self) -> Box<[u8]> {
        match self {
            None => Box::new([]),
            Some(x) => x.to_bytes_representation(),
        }
    }
}