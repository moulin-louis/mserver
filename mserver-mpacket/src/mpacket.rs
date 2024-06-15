use std::{io, vec};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::{Cursor, Read, Write};
use std::net::TcpStream;

use byteorder::{BigEndian, ReadBytesExt};
use uuid::Uuid;
use varint_simd::encode;

use mserialize::MSerialize;
use mserver_types::muuid::Muuid;
use mserver_types::varint::VarInt;

use crate::mpacket::PacketError::NotEnoughByteForPacket;

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
    pub fn new(bytes: &mut Vec<u8>) -> Result<Packet, Box<dyn Error>> {
        let mut cursor = Cursor::new(bytes.clone());

        let length = match leb128::read::signed(&mut cursor) {
            Ok(x) => x,
            Err(_e) => return Err(Box::new(NotEnoughByteForPacket)),
        };
        let (_, bytes_len_length) = encode::<u64>(length as u64);
        if (bytes.len() - bytes_len_length as usize) < length as usize {
            return Err(Box::new(NotEnoughByteForPacket));
        }

        let packet_id = leb128::read::signed(&mut cursor).expect("CANT READ PACKET ID");
        let (_, bytes_packet_id_len) = encode::<u64>(packet_id as u64);

        let offset_data = (bytes_packet_id_len + bytes_len_length) as usize;
        let data_slice = &bytes.as_slice()[offset_data..=(length as usize)];
        let data = Cursor::new(data_slice.to_vec());
        let len_data = data_slice.len();
        bytes.drain(0..offset_data + len_data);
        Ok(Packet {
            length: length.into(),
            packet_id: packet_id.into(),
            data,
        })
    }


    pub fn read_u8(&mut self) -> Result<u8, io::Error> {
        self.data.read_u8()
    }
    pub fn read_u16(&mut self) -> Result<u16, io::Error> {
        self.data.read_u16::<BigEndian>()
    }
    pub fn read_u32(&mut self) -> Result<u32, io::Error> {
        self.data.read_u32::<BigEndian>()
    }
    pub fn read_u64(&mut self) -> Result<u64, io::Error> {
        self.data.read_u64::<BigEndian>()
    }

    pub fn read_varint(&mut self) -> Result<i64, leb128::read::Error> {
        leb128::read::signed(&mut self.data)
    }

    pub fn read_varlong(&mut self) -> Result<i64, leb128::read::Error> {
        leb128::read::signed(&mut self.data)
    }
    pub fn read_bool(&mut self) -> Result<bool, io::Error> {
        match self.read_u8() {
            Ok(x) => Ok(x != 0),
            Err(e) => Err(e),
        }
    }
    pub fn read_string(&mut self) -> Result<String, Box<dyn Error>> {
        let size = self.read_varint()?;
        let mut str_bytes = vec![0u8; size as usize];
        self.data.read_exact(&mut str_bytes)?;
        let res: String = String::from_utf8(str_bytes)?;
        Ok(res)
    }

    pub fn read_uuid(&mut self) -> Result<Muuid, Box<dyn Error>> {
        let mut buff: [u8; 16] = [0; 16];
        self.data.read_exact(&mut buff)?;
        let res = Uuid::from_slice(&buff)?;
        Ok(res.into())
    }


    pub fn send_packet<T>(packet_id: i64, data: &T, tcp_stream: &mut TcpStream) -> Result<(), Box<dyn Error>>
    where
            T: MSerialize,
    {
        let (_, len) = encode::<u64>(packet_id as u64);
        let bytes = data.to_bytes_representation();
        leb128::write::signed(tcp_stream, (len as usize + bytes.len()) as i64).expect("CANT WRITE LEN PACKET");
        leb128::write::signed(tcp_stream, packet_id).expect("CANT WRITE PACKET ID");
        tcp_stream.write_all(bytes.as_ref()).expect("CANT WRITE ALL BYTES");
        println!("packet sent!");
        tcp_stream.flush()?;
        Ok(())
    }

    pub fn send_packet_without_data(packet_id: i64, tcp_stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
        let (_, len) = encode::<u64>(packet_id as u64);
        leb128::write::signed(tcp_stream, len as i64).expect("CANT WRITE LEN PACKET");
        leb128::write::signed(tcp_stream, packet_id).expect("CANT WRITE PACKET ID");
        println!("packet sent!");
        tcp_stream.flush()?;
        Ok(())
    }
}
