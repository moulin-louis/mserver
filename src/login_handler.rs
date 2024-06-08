use std::error::Error;
use bincode::{Encode};
use bincode::enc::Encoder;

use crate::Client;
use crate::mpacket::Packet;
use crate::mstring::MString;
use crate::varint::VarInt;
use mserialize::MSerialize;
use mserialize_derive_macro::MSerialize;
use crate::muuid::MUuid;


#[derive(Encode, MSerialize)]
struct LoginProperty {
    name: MString,
    value: MString,
    is_signed: bool,
    signature: Option<String>,

}

#[derive(Encode, MSerialize)]
struct LoginSuccess {
    uuid: MUuid,
    username: MString,
    nbr_props: VarInt,
    props: LoginProperty,
    strict_error_handling: bool,
}

pub fn cookie_request(_client: &mut Client, packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    let id = packet.read_string().unwrap();
    println!("identifier = {id}");
    Ok(())
}

pub fn login_start(client: &mut Client, packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    client.username = packet.read_string().unwrap();
    println!("username = {}", &client.username);
    client.uuid = packet.read_uuid().unwrap();
    Ok(())
}


pub fn login_success(client: &mut Client, _packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    let login_success = LoginSuccess {
        uuid: client.uuid.into(),
        username: client.username.clone(),
        nbr_props: 0.into(),
        props: LoginProperty {
            name: "toto".into(),
            value: "tata".into(),
            is_signed: false,
            signature: None,
        },
        strict_error_handling: true,
    };
    // client.connection.write_struct(login_success).unwrap();
    Ok(())
}
