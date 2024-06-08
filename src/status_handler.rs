use std::error::Error;
use std::io::Write;

use bincode::Encode;
use byteorder::{BigEndian, ReadBytesExt};
use function_name::named;
use serde::Serialize;

use mserialize::MSerialize;
use mserialize_derive_macro::MSerialize;

use crate::Client;
use crate::mpacket::Packet;
use crate::mstring::MString;

#[derive(Serialize, Encode, MSerialize)]
struct VersionInfo {
    name: String,
    protocol: i32,
}

#[derive(Serialize, Encode, MSerialize)]
struct SampleInfo {
    name: String,
    id: String,
}

#[derive(Serialize, Encode, MSerialize)]
struct PlayerInfo {
    max: i32,
    online: i32,
    sample: Box<[SampleInfo]>,
}

#[derive(Serialize, Encode, MSerialize)]
struct DescriptionInfo {
    text: String,
}

#[derive(Serialize, Encode, MSerialize, )]
struct StatusResponse {
    version: VersionInfo,
    players: PlayerInfo,
    description: DescriptionInfo,
    favicon: String,
    enforcesSecureChat: bool,
    previewsChat: bool,
}

#[named]
pub fn status_response(client: &mut Client, packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    println!("fn = {}", function_name!());
    let status_res = StatusResponse {
        version: VersionInfo {
            name: "1.20.6".to_string(),
            protocol: 766,
        },
        players: PlayerInfo {
            max: 50,
            online: 0,
            sample: Box::new([SampleInfo {
                name: "toto".to_string(),
                id: "1234".to_string(),
            }]),
        },
        description: DescriptionInfo {
            text: "MyServer".to_string(),
        },
        favicon: String::new(),
        enforcesSecureChat: false,
        previewsChat: false,
    };
    let json: MString = serde_json::to_string(&status_res).unwrap().into();
    Packet::send_packet(0x00, &json, &mut client.connection.tcp_stream)
}

//ping response to client from server
#[named]
pub fn ping_response(client: &mut Client, packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    println!("fn = {}", function_name!());
    let payload_id: u64 = packet.data.read_u64::<BigEndian>().expect("CANT READ PAYLOAD ID");
    println!("payloadId = {}", payload_id);
    Packet::send_packet(0x01, &payload_id, &mut client.connection.tcp_stream)
}

#[named]
pub fn status_request(client: &mut Client, packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    println!("fn = {}", function_name!());
    status_response(client, packet)
}

//status request
#[named]
pub fn ping_request(client: &mut Client, packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    println!("fn = {}", function_name!());
    ping_response(client, packet)
}