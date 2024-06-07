use std::{io, thread};
use std::collections::HashMap;
use std::error::Error;
use std::io::ErrorKind::WouldBlock;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use lazy_static::lazy_static;

use mpacket::Packet;

use crate::StateClient::*;

mod handshaking_handler;
mod login_handler;
mod mpacket;

#[derive(Debug, Eq, PartialEq)]
enum StateClient {
    Handshaking,
    Target,
    Status,
    Login,
    Transfer,
}

type HandlerFunction = fn(client: &mut Client, packet: &mut Packet) -> Result<(), Box<dyn Error>>;

lazy_static! {
    static ref HANDSHAKE_HANDLERS: HashMap<i64, HandlerFunction> = {
        let mut m: HashMap<i64, HandlerFunction> = HashMap::new();
        m.insert(0x0, handshaking_handler::set_protocol);
        m
    };
}
lazy_static! {
    static ref TARGET_HANDLERS: HashMap<i64, HandlerFunction> = {
        let mut m: HashMap<i64, HandlerFunction> = HashMap::new();
        m
    };
}
lazy_static! {
    static ref STATUS_HANDLERS: HashMap<i64, HandlerFunction> = {
        let mut m: HashMap<i64, HandlerFunction> = HashMap::new();
        m
    };
}
lazy_static! {
    static ref LOGIN_HANDLERS: HashMap<i64, HandlerFunction> = {
        let mut m: HashMap<i64, HandlerFunction> = HashMap::new();
        m.insert(0x05, login_handler::cookie_request);
        m
    };
}
lazy_static! {
    static ref TRANSFER_HANDLERS: HashMap<i64, HandlerFunction> = {
        let mut m: HashMap<i64, HandlerFunction> = HashMap::new();
        m
    };
}

#[derive(Debug)]
struct Client {
    status: StateClient,
    tcp_stream: TcpStream,
    addr: SocketAddr,
    prot_version: i64,
    server_address: String,
    server_port: u16,
    left_over_packet: Option<Vec<u8>>,
}

impl Client {
    pub fn process_packet(&mut self, packet: &mut Packet) -> Result<(), Box<dyn Error>> {
        println!(
            "packet id = {} for status {:?}",
            packet.packet_id, self.status
        );
        let handler: &HandlerFunction = match self.status {
            Handshaking => HANDSHAKE_HANDLERS
                .get(&packet.packet_id)
                .unwrap_or_else(|| {
                    panic!(
                        "no handle fn for packetID {}: Handshaking",
                        packet.packet_id
                    )
                }),
            Target => HANDSHAKE_HANDLERS
                .get(&packet.packet_id)
                .unwrap_or_else(|| {
                    panic!("no handle fn for packetID {}: Target", packet.packet_id)
                }),
            Status => STATUS_HANDLERS.get(&packet.packet_id).unwrap_or_else(|| {
                panic!("no handle fn for packetID {}: Status", packet.packet_id)
            }),
            Login => LOGIN_HANDLERS
                .get(&packet.packet_id)
                .unwrap_or_else(|| panic!("no handle fn for packetID {}: Login", packet.packet_id)),
            Transfer => TRANSFER_HANDLERS.get(&packet.packet_id).unwrap_or_else(|| {
                panic!("no handle fn for packetID {}: Transfer", packet.packet_id)
            }),
        };
        handler(self, packet).expect("handler packet failed");
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let clients_arc = Arc::new(Mutex::new(Vec::<Client>::new()));
    let clients_arc_clone = Arc::clone(&clients_arc);
    let listener = TcpListener::bind("127.0.0.1:25565").unwrap();
    let client_thread = thread::spawn(move || {
        loop {
            let mut clients = clients_arc.lock().unwrap();
            let mut bad_clients: Vec<usize> = Vec::new();
            for (index, client) in clients.iter_mut().enumerate() {
                let mut packet = match Packet::new(client) {
                    Ok(pck) => pck,
                    Err(e) => {
                        if let Some(io_error) = e.downcast_ref::<io::Error>() {
                            if io_error.kind() == WouldBlock {
                                continue;
                            }
                        }
                        eprintln!("Error when crafting packet: {e}");
                        bad_clients.push(index);
                        continue;
                    }
                };
                client.process_packet(&mut packet).unwrap()
            }
            for &pos in bad_clients.iter().rev() {
                //removing client  that give an error before
                clients.remove(pos);
            }
            drop(clients);
            sleep(Duration::from_millis(800));
        }
    });
    loop {
        let (tcp_stream, addr) = match listener.accept() {
            Ok(res) => res,
            Err(e) => {
                eprintln!("Error when accepting: {}", e);
                break;
            }
        };
        let client: Client = Client {
            status: Handshaking,
            tcp_stream,
            addr,
            prot_version: 0,
            server_address: String::new(),
            server_port: 0,
            left_over_packet: None,
        };
        client.tcp_stream.set_nonblocking(true).unwrap();
        clients_arc_clone.lock().unwrap().push(client);
    }
    client_thread.join().unwrap();
    Ok(())
}
