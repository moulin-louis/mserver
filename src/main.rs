use std::collections::HashMap;
use std::error::Error;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use bevy_ecs::world::World;
use local_ip_address::linux::local_ip;
use uuid::Uuid;

use mserver_client::client::{Client, ClientInfo};
use mserver_client::state::StateClient;
use mserver_mpacket::mpacket::{Packet, PacketError};

fn handle_clients(clients_arc: Arc<Mutex<Vec<Client>>>) {
    loop {
        let mut clients = clients_arc.lock().unwrap();
        let mut bad_clients: Vec<usize> = Vec::new();
        for (index, client) in clients.iter_mut().enumerate() {
            match client.fetch_bytes() {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("error when reading from tcpstream {e}");
                    bad_clients.push(index);
                    continue;
                }
            }
            let mut packet = match Packet::new(&mut client.bytes) {
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

    let world = Mutex::new(World::new());
    let world_ref = Arc::new(world);
    let client_thread = thread::spawn(|| {
        handle_clients(clients_arc);
    });
    let my_local_ip = local_ip().unwrap();
    println!("my local ip = {}", my_local_ip);
    let listener = TcpListener::bind(my_local_ip.to_string() + ":25565").unwrap();
    loop {
        let (tcp_stream, addr) = match listener.accept() {
            Ok(res) => res,
            Err(e) => {
                eprintln!("Error when accepting: {}", e);
                break;
            }
        };
        let client: Client = Client {
            world: world_ref.clone(),
            status: StateClient::Handshaking,
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
