use std::collections::HashMap;
use std::error::Error;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use lazy_static::lazy_static;
use uuid::Uuid;

use crate::client::{Client, ClientInfo};
use crate::mpacket::{Packet, PacketError};
use crate::StateClient::*;

mod handshaking_handler;
mod login_handler;
mod status_handler;
mod mpacket;
mod varint;
mod varlong;
mod connection;
mod muuid;
mod configuration_handlers;
mod client;
mod play_handler;


#[derive(Debug, Eq, PartialEq)]
enum StateClient {
    Handshaking,
    Status,
    Login,
    Configuration,
    Play,
    Target,
    Transfer,
}

type HandlerFunction = fn(client: &mut Client, packet: &mut Packet) -> Result<(), Box<dyn Error>>;

lazy_static! {
    static ref HANDSHAKE_HANDLERS: HashMap<i64, HandlerFunction> = {
        let mut m: HashMap<i64, HandlerFunction> = HashMap::new();
        m.insert(0x0, handshaking_handler::set_protocol);
        m
    };
    static ref STATUS_HANDLERS: HashMap<i64, HandlerFunction> = {
        let mut m: HashMap<i64, HandlerFunction> = HashMap::new();
        m.insert(0x00, status_handler::status_request);
        m.insert(0x01, status_handler::ping_request);
        m
    };
    static ref LOGIN_HANDLERS: HashMap<i64, HandlerFunction> = {
        let mut m: HashMap<i64, HandlerFunction> = HashMap::new();
        m.insert(0x00, login_handler::login_start);
        m.insert(0x03, login_handler::login_ack);
        m.insert(0x05, login_handler::cookie_request);
        m
    };
    static ref CONFIGURATION_HANDLERS: HashMap<i64, HandlerFunction> = {
        let mut m: HashMap<i64, HandlerFunction> = HashMap::new();
        m.insert(0x00, configuration_handlers::client_info);
        m.insert(0x02, configuration_handlers::serv_plugin_message);
        m.insert(0x03, configuration_handlers::finish_config_ack);
        m
    };
    static ref PLAY_HANDLERS: HashMap<i64, HandlerFunction> = {
        let m: HashMap<i64, HandlerFunction> = HashMap::new();
        m
    };
    static ref TARGET_HANDLERS: HashMap<i64, HandlerFunction> = {
        let m: HashMap<i64, HandlerFunction> = HashMap::new();
        m
    };
    static ref TRANSFER_HANDLERS: HashMap<i64, HandlerFunction> = {
        let m: HashMap<i64, HandlerFunction> = HashMap::new();
        m
    };
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
            server_address: String::new(),
            server_port: 0,
            username: String::new(),
            uuid: Uuid::nil().into(),
            client_info: ClientInfo::default(),
            plugin_message: HashMap::new(),
            bytes: Vec::new(),
        };
        client.tcp_stream.set_nonblocking(true).unwrap();
        clients_arc_clone.lock().unwrap().push(client);
        println!("adding one client");
    }
    client_thread.join().unwrap();
    Ok(())
}
