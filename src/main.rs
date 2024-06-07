use std::net::TcpListener;
use std::thread;

use leb128;

use mpacket::Packet;

mod mpacket;

enum StateClient {
    Handshaking,
    Target,
    Status,
    Login,
    Transfer,
}

struct client {
    status: StateClient,
    server_address: String,
    server_port: u16,
    protocol_version: i32,
}

impl client {
    pub fn process_packet(packet: Packet) {}
}

fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("localhost:25565").unwrap();
    println!("listener created {:?}", listener);
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        thread::spawn(|| {
            let packet = Packet::new(&mut stream);
            client::process_packet(packet)
        });
    }
}
