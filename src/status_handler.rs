use std::error::Error;

use byteorder::{BigEndian, ReadBytesExt};
use function_name::named;
use serde::Serialize;

use crate::Client;
use crate::mpacket::Packet;
use crate::mstring::MString;

#[derive(Serialize)]
struct VersionInfo {
    name: String,
    protocol: i32,
}

#[derive(Serialize)]
struct SampleInfo {
    name: String,
    id: String,
}

#[derive(Serialize)]
struct PlayerInfo {
    max: i32,
    online: i32,
    sample: Box<[SampleInfo]>,
}

#[derive(Serialize)]
struct DescriptionInfo {
    text: String,
}

#[derive(Serialize)]
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
    Packet::send_packet(0x00, &json, &mut client.tcp_stream)
}

//ping response to client from server
pub fn ping_response(client: &mut Client, packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    let payload_id: u64 = packet.data.read_u64::<BigEndian>().expect("CANT READ PAYLOAD ID");
    Packet::send_packet(0x01, &payload_id, &mut client.tcp_stream)
}

pub fn status_request(client: &mut Client, packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    status_response(client, packet)
}

//status request
pub fn ping_request(client: &mut Client, packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    ping_response(client, packet)
}