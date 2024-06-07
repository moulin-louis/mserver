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

mod mpacket;

trait PacketHandler {
    fn handle_packet(&self, client: &mut Client, packet: &Packet) -> Result<(), Box<dyn Error>>;
}

struct TargetHandler;
impl PacketHandler for TargetHandler {
    fn handle_packet(&self, client: &mut Client, packet: &Packet) -> Result<(), Box<dyn Error>> {
        // Handle Target packets
        println!("Target packet handler: {:?}", packet);
        Ok(())
    }
}

lazy_static! {
    static ref HANDSHAKING_HANDLERS: HashMap<i64, Box<dyn PacketHandler>> = {
        let mut m = HashMap::new();
        m.insert(0, Box::new(HandshakingHandler));
        // Add more handlers for different packet IDs
        m
    };

    static ref TARGET_HANDLERS: HashMap<i64, Box<dyn PacketHandler>> = {
        let mut m = HashMap::new();
        m.insert(1, Box::new(TargetHandler));
        // Add more handlers for different packet IDs
        m
    };

    // Define other state handlers similarly...
}

#[derive(Debug, Eq, PartialEq)]
enum StateClient {
    Handshaking,
    Target,
    Status,
    Login,
    Transfer,
}


#[derive(Debug)]
struct Client {
    status: StateClient,
    tcp_stream: TcpStream,
    addr: SocketAddr,
}


impl Client {
    pub fn process_packet(&mut self, packet: &Packet) -> Result<(), Box<dyn Error>> {
        match self.status {
            StateClient::Handshaking => {}
            StateClient::Target => {}
            StateClient::Status => {}
            StateClient::Login => {}
            StateClient::Transfer => {}
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut handlers: HashMap<i64, Box<dyn PacketHandler>> = HashMap::new();
    let clients_arc = Arc::new(Mutex::new(Vec::<Client>::new()));
    let clients_arc_clone = Arc::clone(&clients_arc);
    let listener = TcpListener::bind("localhost:25565").unwrap();
    let client_thread = thread::spawn(move || {
        loop {
            let mut clients = clients_arc.lock().unwrap();
            let mut bad_clients: Vec<usize> = Vec::new();
            for (index, client) in clients.iter_mut().enumerate() {
                let packet = match Packet::new(&mut client.tcp_stream) {
                    Ok(pck) => pck,
                    Err(e) => {
                        if let Some(io_error) = e.downcast_ref::<io::Error>() {
                            match io_error.kind() {
                                WouldBlock => continue,
                                _ => {}
                            }
                        }
                        eprintln!("Error when crafting packet: {e}");
                        bad_clients.push(index);
                        continue;
                    }
                };
                client.process_packet(&packet).unwrap()
            }
            for &pos in bad_clients.iter().rev() { //removing client  that give an error before
                eprintln!("removing client from vec");
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
        println!("adding client ");
        let client: Client = Client {
            status: StateClient::Handshaking,
            tcp_stream,
            addr,
        };
        client.tcp_stream.set_nonblocking(true).unwrap();
        clients_arc_clone.lock().unwrap().push(client);
    }
    client_thread.join().unwrap();
    return Ok(());
}
