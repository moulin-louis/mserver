use std::error::Error;
use std::io::ErrorKind::WouldBlock;
use std::io::{Cursor, Read};

use uuid::Uuid;

use crate::mstring::MString;
use crate::varint::VarInt;
use crate::Client;

#[derive(Debug)]
pub struct Packet {
    pub length: VarInt,    //varInt
    pub packet_id: VarInt, //varInt
    pub data: Cursor<Vec<u8>>,
}

impl Packet {
    pub fn new(client: &mut Client) -> Result<Packet, Box<dyn Error>> {
        loop {
            let mut buf = [0u8; 1024];
            let bytes_read = match client.connection.tcp_stream.read(&mut buf) {
                Ok(x) => x,
                Err(e) => {
                    if e.kind() == WouldBlock {
                        break;
                    }
                    return Err(Box::new(e));
                }
            };
            client.bytes_packet.append(&mut buf[0..bytes_read].to_vec());
        }
        // println!("bytes packet = {:?}", &client.bytes_packet);
        let mut data = Cursor::new(client.bytes_packet.to_owned());
        client.bytes_packet.clear();
        let length: VarInt = leb128::read::signed(&mut data)?.into();
        let packet_id = leb128::read::signed(&mut data)?.into();
        Ok(Packet {
            length,
            packet_id,
            data,
        })
    }

    pub fn return_leftover(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut result: Vec<u8> = Vec::new();
        self.data.read_to_end(&mut result)?;
        Ok(result)
    }

    pub fn read_string(&mut self) -> Result<MString, Box<dyn Error>> {
        let size = leb128::read::signed(&mut self.data).unwrap();
        let mut address_bytes = vec![0u8; size as usize];
        self.data
            .read_exact(&mut address_bytes)
            .expect("cannot read full ip string");
        let res: MString = address_bytes.into();
        Ok(res)
    }

    pub fn read_uuid(&mut self) -> Result<Uuid, Box<dyn Error>> {
        let mut buff: [u8; 16] = [0; 16];
        self.data.read_exact(&mut buff)?;
        let res = Uuid::from_slice(&buff)?;
        return Ok(res);
    }
}
