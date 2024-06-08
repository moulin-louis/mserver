use std::error::Error;

use function_name::named;
use serde::Serialize;

use crate::mpacket::Packet;
use crate::Client;

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
    let statusRes = StatusResponse {
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
    println!("fn = {}", function_name!());
    Ok(())
}

//ping response to client from server
#[named]
pub fn pingToClient(client: &mut Client, packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    println!("fn = {}", function_name!());
    Ok(())
}

//status request
#[named]
pub fn ping_start(client: &mut Client, packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    println!("fn = {}", function_name!());
    status_response(client, packet)
}

#[named]
pub fn status_request(client: &mut Client, packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    println!("fn = {}", function_name!());
    return status_response(client, packet);
}
