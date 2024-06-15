use std::collections::HashMap;
use std::error::Error;
use std::io::ErrorKind::WouldBlock;
use std::io::Read;
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};

use bevy_ecs::world::World;

use mserver_mpacket::mpacket::Packet;
use mserver_types::muuid::Muuid;
use mserver_types::varint::VarInt;

use crate::state::{HandlerFunction, StateClient};
use crate::state::*;

#[derive(Debug, Default)]
pub struct ClientInfo {
    pub locale: String,
    pub view_distance: u8,
    pub chat_mode: VarInt,
    pub chat_color: bool,
    pub displayed_skin_parts: u8,
    pub main_hand: VarInt,
    pub text_filtering: bool,
    pub server_listings: bool,
}

#[derive(Debug)]
pub struct Client {
    pub world: Arc<Mutex<World>>,
    pub status: StateClient,
    pub tcp_stream: TcpStream,
    pub addr: SocketAddr,
    pub prot_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub username: String,
    pub uuid: Muuid,
    pub client_info: ClientInfo,
    pub plugin_message: HashMap<String, String>,
    pub bytes: Vec<u8>,
}

impl Client {
    pub fn process_packet(&mut self, packet: &mut Packet) -> Result<(), Box<dyn Error>> {
        println!(
            "packet id = {:#02x} for status {:?}",
            packet.packet_id.0, self.status
        );
        let handler: &HandlerFunction = match self.status {
            StateClient::Handshaking => HANDSHAKE_HANDLERS.get(&packet.packet_id.get_val()).unwrap_or_else(|| {
                panic!("no handle fn for packetID {}: Handshaking", packet.packet_id)
            }),
            StateClient::Status => STATUS_HANDLERS.get(&packet.packet_id.get_val()).unwrap_or_else(|| {
                panic!("no handle fn for packetID {}: Status", packet.packet_id)
            }),
            StateClient::Login => LOGIN_HANDLERS.get(&packet.packet_id.get_val()).unwrap_or_else(|| {
                panic!("no handle fn for packetID {}: Login", packet.packet_id)
            }),
            StateClient::Configuration => CONFIGURATION_HANDLERS.get(&packet.packet_id.get_val()).unwrap_or_else(|| {
                panic!("no handle fn for packetID {:#02X}: Configuration", packet.packet_id.0)
            }),
            StateClient::Play => PLAY_HANDLERS.get(&packet.packet_id.get_val()).unwrap_or_else(|| {
                panic!("no handle fn for packetID {:#02X}: Play", packet.packet_id.0)
            }),
            StateClient::Target => TARGET_HANDLERS.get(&packet.packet_id.get_val()).unwrap_or_else(|| {
                panic!("no handle fn for packetID {}: Target", packet.packet_id)
            }),
            StateClient::Transfer => TRANSFER_HANDLERS.get(&packet.packet_id.get_val()).unwrap_or_else(|| {
                panic!("no handle fn for packetID {}: Transfer", packet.packet_id)
            }),
        };
        handler(self, packet).expect("handler packet failed");
        Ok(())
    }

    pub fn fetch_bytes(&mut self) -> Result<usize, Box<dyn Error>> {
        let mut total_bytes_read = 0;
        loop {
            let mut buff: [u8; 1] = [0];
            match self.tcp_stream.read_exact(&mut buff) {
                Ok(_) => {
                    total_bytes_read += 1;
                    self.bytes.push(buff[0]);
                }
                Err(e) => {
                    if e.kind() == WouldBlock {
                        break;
                    }
                    return Err(Box::new(e));
                }
            }
        }
        Ok(total_bytes_read)
    }
}