use std::error::Error;

use function_name::named;

use crate::client::Client;
use crate::mpacket::Packet;

#[named]
fn tmp(client: &mut Client, packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    println!("called fn {}", function_name!());
    Ok(())
}