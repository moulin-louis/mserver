use std::error::Error;
use std::io::Read;

use mserver_mpacket::mpacket::Packet;

use crate::client::Client;
use crate::state::StateClient::{Login, Status, Target, Transfer};

pub fn set_protocol(client: &mut Client, packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    client.status = Target;
    client.prot_version = leb128::read::signed(&mut packet.data).expect("cant read prot version").into();
    client.server_address = packet.read_string().unwrap();
    let mut server_port: [u8; 2] = [0; 2];
    packet.data.read_exact(&mut server_port).unwrap();
    client.server_port = ((server_port[0] as u16) << 8) | server_port[1] as u16;
    client.status = match leb128::read::signed(&mut packet.data).unwrap() {
        1 => Status,
        2 => Login,
        3 => Transfer,
        _err => panic!("PANIC WRONG NEXT STATE: got {_err}"),
    };
    Ok(())
}
