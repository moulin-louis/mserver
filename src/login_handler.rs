use std::error::Error;

use bincode::Encode;
use function_name::named;

use mserialize_derive_macro::MSerialize;

use crate::{Client, StateClient};
use crate::mpacket::Packet;
use crate::muuid::Muuid;
use crate::varint::VarInt;

// #[derive(Encode, MSerialize)]
// struct LoginProperty {
//     name: String,
//     value: String,
//     is_signed: bool,
//     signature: Option<String>,
// }

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

#[named]
pub fn login_ack(client: &mut Client, packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    println!("called fn {}", function_name!());
    client.status = StateClient::Configuration;
    Ok(())
}