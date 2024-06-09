use std::error::Error;

use function_name::named;

use crate::{Client, StateClient};
use crate::client::ClientInfo;
use crate::mpacket::Packet;

pub fn client_info(client: &mut Client, packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    let client_info = ClientInfo {
        locale: packet.read_string().unwrap(),
        view_distance: packet.read_u8().unwrap(),
        chat_mode: packet.read_varint().unwrap().into(),
        chat_color: packet.read_bool().unwrap(),
        displayed_skin_parts: packet.read_u8().unwrap(),
        main_hand: packet.read_varint().unwrap().into(),
        text_filtering: packet.read_bool().unwrap(),
        server_listings: packet.read_bool().unwrap(),
    };
    println!("client info = {:?}", client_info);
    client.client_info = client_info;
    //registry data
    //finish config
    Packet::send_packet_without_data(0x03, &mut client.tcp_stream).unwrap();
    Ok(())
}

pub fn serv_plugin_message(client: &mut Client, packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    let id = packet.read_string().unwrap();
    let data = packet.read_string().unwrap();
    client.plugin_message.insert(id, data);
    Ok(())
}

#[named]
pub fn finish_config_ack(client: &mut Client, packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    println!("called fn {}", function_name!());
    println!("CONFIGURATION IS DONE!");
    client.status = StateClient::Play;
    Ok(())
}