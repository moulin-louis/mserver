use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::{Cursor, Read, Write};
use std::net::TcpStream;

use uuid::Uuid;
use varint_simd::encode;

use mserialize::MSerialize;

use crate::Client;
use crate::mstring::MString;
use crate::varint::VarInt;

#[derive(Debug)]
pub struct Packet {
    pub length: VarInt,    //varInt
    pub packet_id: VarInt, //varInt
    pub data: Cursor<Vec<u8>>,
}

#[derive(Debug)]
pub enum PacketError {
    NotEnoughByteForPacket,
}

impl Display for PacketError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for PacketError {}


impl Packet {
    pub fn new(client: &mut Client) -> Result<Packet, Box<dyn Error>> {
        let length: VarInt = leb128::read::signed(&mut client.connection.tcp_stream).unwrap().into();
        let mut buff: Vec<u8> = vec![0; length.get_val() as usize];
        client.connection.tcp_stream.read_exact(buff.as_mut_slice()).expect("Enable to read all the data at once");
        let mut data = Cursor::new(buff);
        let packet_id: VarInt = leb128::read::signed(&mut data).unwrap().into();
        Ok(Packet {
            length,
            packet_id,
            data: data,
        })
    }


    pub fn from(length: VarInt, packet_id: VarInt, data: Cursor<Vec<u8>>) -> Packet {
        Packet {
            length,
            packet_id,
            data,
        }
    }

    pub fn read_string(&mut self) -> Result<MString, Box<dyn Error>> {
        let size = leb128::read::signed(&mut self.data).unwrap();
        let mut address_bytes = vec![0u8; size as usize];
        self.data.read_exact(&mut address_bytes).expect("cannot read full ip string");
        let res: MString = address_bytes.into();
        Ok(res)
    }

    pub fn read_uuid(&mut self) -> Result<Uuid, Box<dyn Error>> {
        let mut buff: [u8; 16] = [0; 16];
        self.data.read_exact(&mut buff)?;
        let res = Uuid::from_slice(&buff)?;
        return Ok(res);
    }

    pub fn send_packet<T>(packet_id: i64, data: &T, tcp_stream: &mut TcpStream) -> Result<(), Box<dyn Error>>
        where T: MSerialize {
        let bytes = data.to_bytes_representation();
        let (_, len) = encode::<u64>(packet_id as u64);
        leb128::write::signed(tcp_stream, (len as usize + bytes.len()) as i64).expect("CANT WRITE LEN PACKET");
        leb128::write::signed(tcp_stream, packet_id).expect("CANT WRITE PACKET ID");
        tcp_stream.write_all(bytes.as_ref()).expect("CANT WRITE ALL BYTES");
        Ok(())
    }
}
