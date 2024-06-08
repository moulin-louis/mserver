use std::collections::HashMap;
use std::error::Error;
use std::io::ErrorKind::WouldBlock;
use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use lazy_static::lazy_static;
use uuid::Uuid;

use crate::mpacket::{Packet, PacketError};
use crate::mstring::MString;
use crate::StateClient::*;
use crate::varint::VarInt;

mod handshaking_handler;
mod login_handler;
mod status_handler;
mod mpacket;
mod mstring;
mod varint;
mod varlong;
mod connection;
mod muuid;


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
    static ref HANDSHAKE_HANDLERS_FROMCLIENT: HashMap<i64, HandlerFunction> = {
        let mut m: HashMap<i64, HandlerFunction> = HashMap::new();
        m.insert(0x0, handshaking_handler::set_protocol);
        m
    };
    static ref TARGET_HANDLERS_FROMCLIENT: HashMap<i64, HandlerFunction> = {
        let m: HashMap<i64, HandlerFunction> = HashMap::new();
        m
    };
    // static ref TARGET_HANDLERS_TOCLIENT: HashMap<i64, HandlerFunction> = {
    //     let m: HashMap<i64, HandlerFunction> = HashMap::new();
    //     m
    // };
    static ref STATUS_HANDLERS_FROMCLIENT: HashMap<i64, HandlerFunction> = {
        let mut m: HashMap<i64, HandlerFunction> = HashMap::new();
        m.insert(0x00, status_handler::status_request);
        m.insert(0x01, status_handler::ping_request);
        m
    };
    // static ref STATUS_HANDLERS_TOCLIENT: HashMap<i64, HandlerFunction> = {
    //     let mut m: HashMap<i64, HandlerFunction> = HashMap::new();
    //     m.insert(0x00, status_handler::status_response);
    //     m.insert(0x01, status_handler::ping_to_client);
    //     m
    // };
    static ref LOGIN_HANDLERS_FROMCLIENT: HashMap<i64, HandlerFunction> = {
        let mut m: HashMap<i64, HandlerFunction> = HashMap::new();
        m.insert(0x0, login_handler::login_start);
        m.insert(0x05, login_handler::cookie_request);
        m
    };
    // static ref LOGIN_HANDLERS_TOCLIENT: HashMap<i64, HandlerFunction> = {
    //     let mut m: HashMap<i64, HandlerFunction> = HashMap::new();
    //     m.insert(0x02, login_handler::login_success);
    //     m
    // };
    static ref TRANSFER_HANDLERS_FROMCLIENT: HashMap<i64, HandlerFunction> = {
        let m: HashMap<i64, HandlerFunction> = HashMap::new();
        m
    };
    // static ref TRANSFER_HANDLERS_TOCLIENT: HashMap<i64, HandlerFunction> = {
    //     let m: HashMap<i64, HandlerFunction> = HashMap::new();
    //     m
    // };
}



#[derive(Debug)]
struct Client {
    status: StateClient,
    tcp_stream: TcpStream,
    addr: SocketAddr,
    prot_version: VarInt,
    server_address: MString,
    server_port: u16,
    username: MString,
    uuid: Uuid,
    bytes: Vec<u8>,
    // cursor_packet: Cursor<Vec<u8>>,
}

impl Client {
    pub fn process_packet(&mut self, packet: &mut Packet) -> Result<(), Box<dyn Error>> {
        println!(
            "packet id = {} for status {:?}",
            packet.packet_id, self.status
        );
        let handler: &HandlerFunction = match self.status {
            Handshaking => HANDSHAKE_HANDLERS_FROMCLIENT.get(&packet.packet_id.get_val()).unwrap_or_else(|| {
                panic!(
                    "no handle fn for packetID {}: Handshaking",
                    packet.packet_id
                )
            }),
            Target => TARGET_HANDLERS_FROMCLIENT.get(&packet.packet_id.get_val()).unwrap_or_else(|| {
                panic!("no handle fn for packetID {}: Target", packet.packet_id)
            }),
            Status => STATUS_HANDLERS_FROMCLIENT.get(&packet.packet_id.get_val()).unwrap_or_else(|| {
                panic!("no handle fn for packetID {}: Status", packet.packet_id)
            }),
            Login => LOGIN_HANDLERS_FROMCLIENT.get(&packet.packet_id.get_val()).unwrap_or_else(|| panic!("no handle fn for packetID {}: Login", packet.packet_id)),
            Transfer => TRANSFER_HANDLERS_FROMCLIENT.get(&packet.packet_id.get_val()).unwrap_or_else(|| {
                panic!("no handle fn for packetID {}: Transfer", packet.packet_id)
            }),
        };
        handler(self, packet).expect("handler packet failed");
        Ok(())
    }

    pub fn fetch_bytes(&mut self) -> Result<usize, Box<dyn Error>> {
        let mut total_bytes_read = 0;
        loop {
            let mut buff: [u8; 1024] = [0; 1024];
            match self.tcp_stream.read(&mut buff) {
                Ok(x) => {
                    if x == 0 {
                        break;
                    }
                    total_bytes_read += x;
                    self.bytes.append(&mut buff[0..=x].to_vec());
                }
                Err(e) => {
                    if e.kind() == WouldBlock {
                        break;
                    }
                    return Err(Box::new(e));
                }
            }
        }
        // println!("done reading {} bytes", total_bytes_read);
        println!("current byte buff = {:02X?}", self.bytes);
        Ok(total_bytes_read)
    }
}

fn handle_clients(clients_arc: Arc<Mutex<Vec<Client>>>) {
    loop {
        let mut clients = clients_arc.lock().unwrap();
        let mut bad_clients: Vec<usize> = Vec::new();
        for (index, client) in clients.iter_mut().enumerate() {
            client.fetch_bytes().unwrap();
            let mut packet = match Packet::new(client) {
                Ok(pck) => pck,
                Err(e) => {
                    if e.is::<PacketError>() {
                        // eprintln!("packeterror, not enough byte to make a packet, continuing");
                        continue;
                    }
                    eprintln!("Error when crafting packet: {e}");
                    bad_clients.push(index);
                    continue;
                }
            };
            client.process_packet(&mut packet).unwrap();
        }
        for &pos in bad_clients.iter().rev() {
            println!("removing one client");
            clients.remove(pos);
        }
        drop(clients);
        sleep(Duration::from_millis(100));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let clients_arc = Arc::new(Mutex::new(Vec::<Client>::new()));
    let clients_arc_clone = Arc::clone(&clients_arc);
    let listener = TcpListener::bind("127.0.0.1:25565").unwrap();
    let client_thread = thread::spawn(move || {
        handle_clients(clients_arc);
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
            prot_version: 0.into(),
            server_address: MString::new(),
            server_port: 0,
            username: MString::new(),
            uuid: Uuid::nil(),
            bytes: Vec::new(),
        };
        client.tcp_stream.set_nonblocking(true).unwrap();
        clients_arc_clone.lock().unwrap().push(client);
        println!("adding one client");
    }
    client_thread.join().unwrap();
    Ok(())
}
