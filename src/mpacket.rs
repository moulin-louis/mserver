use std::io::Read;
use std::net::TcpStream;

pub struct Packet {
    length: i64,    //varInt
    packet_id: i64, //varInt
    data: Vec<u8>,
}

impl Packet {
    pub fn new(tcp_stream: &mut TcpStream) -> Packet {
        let length = leb128::read::signed(tcp_stream).unwrap();
        let packet_id = leb128::read::signed(tcp_stream).unwrap();
        let mut data = Vec::new();
        data.resize(length as usize, 0);
        tcp_stream
            .read_exact(&mut data[0..length as usize])
            .unwrap();
        Packet {
            length,
            packet_id,
            data,
        }
    }
}
