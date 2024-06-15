use std::error::Error;

use bincode::Encode;

use mserialize_derive_macro::MSerialize;
use mserver_mpacket::mpacket::Packet;
use mserver_types::muuid::Muuid;
use mserver_types::varint::VarInt;

use crate::client::Client;
use crate::state::StateClient;

#[derive(Encode, MSerialize)]
struct LoginSuccess {
    uuid: Muuid,
    username: String,
    nbr_props: VarInt,
    strict_error_handling: bool,
}

pub fn cookie_request(_client: &mut Client, packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    let id = packet.read_string().unwrap();
    println!("identifier = {id}");
    Ok(())
}

pub fn login_start(client: &mut Client, packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    client.username = packet.read_string().unwrap();
    client.uuid = packet.read_uuid().unwrap();
    login_success(client, packet)
}

pub fn login_success(client: &mut Client, _packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    let login_success = LoginSuccess {
        uuid: client.uuid,
        username: client.username.clone(),
        nbr_props: 0.into(),
        strict_error_handling: true,
    };
    // login_success.to_bytes_representation();
    Packet::send_packet(0x02, &login_success, &mut client.tcp_stream).unwrap();
    Ok(())
}

pub fn login_ack(client: &mut Client, _packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    client.status = StateClient::Configuration;
    Ok(())
}