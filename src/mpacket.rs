use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::{Cursor, Read, Write};
use std::net::TcpStream;

use uuid::Uuid;
use varint_simd::encode;

use mserialize::MSerialize;

use crate::Client;
use crate::mpacket::PacketError::NotEnoughByteForPacket;
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

impl Error for PacketError {}

impl Display for PacketError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


impl Packet {
    pub fn new(client: &mut Client) -> Result<Packet, Box<dyn Error>> {
        if client.bytes.len() < 3 {
            return Err(Box::new(NotEnoughByteForPacket));
        }

        let mut cursor = Cursor::new(client.bytes.clone());

        let length = leb128::read::signed(&mut cursor).expect("CANT READ LENGTH PACKET");
        let (_, bytes_len_length) = encode::<u64>(length as u64);

        if (client.bytes.len() - bytes_len_length as usize) < length as usize {
            return Err(Box::new(NotEnoughByteForPacket));
        }

        let packet_id = leb128::read::signed(&mut cursor).expect("CANT READ PACKET ID");
        let (_, bytes_packet_id_len) = encode::<u64>(packet_id as u64);

        let offset_data = (bytes_packet_id_len + bytes_len_length) as usize;
        let data_slice = &client.bytes.as_slice()[offset_data..=(length as usize)];
        let data = Cursor::new(data_slice.to_vec());
        let len_data = if data_slice.len() == 0 { 1 } else { data_slice.len() };
        client.bytes.drain(0..offset_data + len_data);
        println!("bytes left = {:02X?}", client.bytes);
        Ok(Packet {
            length: length.into(),
            packet_id: packet_id.into(),
            data,
        })
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
        println!("sending: {:02x?}", bytes);
        let (_, len) = encode::<u64>(packet_id as u64);
        leb128::write::signed(tcp_stream, (len as usize + bytes.len()) as i64).expect("CANT WRITE LEN PACKET");
        leb128::write::signed(tcp_stream, packet_id).expect("CANT WRITE PACKET ID");
        tcp_stream.write_all(bytes.as_ref()).expect("CANT WRITE ALL BYTES");
        println!("packet sent!");
        tcp_stream.flush()?;
        Ok(())
    }
}
