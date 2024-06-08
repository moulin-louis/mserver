use std::error::Error;
use std::io::Write;
use std::net::TcpStream;

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

// Implement the trait for multiple types using the macro
impl_for_primitives! { i32 u32 i64 u64 f32 f64 }

// fn test() {
//     let i = 0;
//     i.to_be_bytes().as
// }