use std::error::Error;
use std::io::{Cursor, Read};

use crate::Client;

#[derive(Debug)]
pub struct Packet {
    pub length: i64,    //varInt
    pub packet_id: i64, //varInt
    pub data: Cursor<Vec<u8>>,
}

impl Packet {
    pub fn new(client: &mut Client) -> Result<Packet, Box<dyn Error>> {
        let length = leb128::read::signed(&mut client.tcp_stream)?;
        println!("length packet = {}", length);
        let packet_id = leb128::read::signed(&mut client.tcp_stream)?;
        let mut data = vec![0; length as usize];
        client
            .tcp_stream
            .read_exact(&mut data[0..length as usize])?;
        let data = match &client.left_over_packet {
            None => Cursor::new(data),
            Some(x) => Cursor::new([x, data.as_slice()].concat()),
        };
        Ok(Packet {
            length,
            packet_id,
            data,
        })
    }
    pub fn read_string(&mut self) -> Result<String, Box<dyn Error>> {
        let size = leb128::read::signed(&mut self.data).unwrap();
        println!("size str = {}", size);
        let mut address_bytes = vec![0u8; size as usize];
        self.data
            .read_exact(&mut address_bytes)
            .expect("cannot read full ip string");
        let res = String::from_utf8(address_bytes)?;
        Ok(res)
    }
}
