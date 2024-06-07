use std::error::Error;

use crate::Client;
use crate::mpacket::Packet;

pub fn set_protocol() {}

pub fn cookie_request(client: &mut Client, packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    let id = packet.read_string().unwrap();
    println!("identifier = {id}");
    Ok(())
}
