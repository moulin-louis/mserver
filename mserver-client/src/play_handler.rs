use std::error::Error;

use bincode::enc::Encoder;
use bincode::Encode;
use uuid::Uuid;

use mserialize::MSerialize;
use mserialize_derive_macro::MSerialize;
use mserver_entity::MEntity;
use mserver_mpacket::mpacket::Packet;
use mserver_types::varint::VarInt;

use crate::client::Client;

#[derive(Debug, Encode, MSerialize)]
struct DeathInfo {
    dimension_name: String,
    location: u64,
}

#[derive(Debug, Encode, MSerialize)]
struct LoginInfo {
    entity_id: u32,
    is_hardcode: bool,
    dimensions_count: VarInt,
    dimensions_names: Vec<String>,
    max_players: VarInt, //ignored
    view_distance: VarInt,
    simulation_distance: VarInt,
    reduced_debug_info: bool,
    respawn_screen: bool,
    limited_crafting: bool,
    dimension_type: VarInt,
    dimension_name: String,
    hashed_seed: u64,
    game_mode: u8,
    prev_game_mode: i8,
    is_debug: bool,
    is_flat: bool,
    has_death_location: bool,
    portal_cooldown: VarInt,
    secure_chat: bool,
}

pub fn login(client: &mut Client, _packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    //spawn the player into the ECS
    let mut world = client.world.lock().unwrap();
    let player_entity = world.spawn(MEntity {
        uuid: Uuid::new_v4(),
        r#type: 122.into(),
        x: 0.0,
        y: 0.0,
        z: 0.0,
        pitch: 0,
        yaw: 0,
        head_yaw: 0,
        data: 0.into(),
        vx: 0,
        vy: 0,
        vz: 0,
    });
    let player_entity_id = player_entity.id();
    println!("entity id = {}", player_entity_id.index());
    let login_info = LoginInfo {
        entity_id: player_entity_id.index(),
        is_hardcode: false,
        dimensions_count: 1.into(),
        dimensions_names: vec!["overworld".to_string()],
        max_players: 50.into(),
        view_distance: 2.into(),
        simulation_distance: 2.into(),
        reduced_debug_info: false,
        respawn_screen: true,
        limited_crafting: false,
        dimension_type: 0.into(),
        dimension_name: "overworld".to_string(),
        hashed_seed: 0,
        game_mode: 0,
        prev_game_mode: -1,
        is_debug: true,
        is_flat: false,
        has_death_location: false,
        portal_cooldown: 1.into(),
        secure_chat: false,
    };
    println!("size of login info = {}", std::mem::size_of_val(&login_info));
    println!("size byte of entity_id = {}", player_entity_id.index().size_bytes());
    println!("size byte of is_hardcore = {}", false.size_bytes());
    println!("size byte of entity_id = {}", player_entity_id.index().size_bytes());
    println!("size byte of entity_id = {}", player_entity_id.index().size_bytes());
    println!("size byte of entity_id = {}", player_entity_id.index().size_bytes());
    println!("size byte of entity_id = {}", player_entity_id.index().size_bytes());
    println!("size byte of entity_id = {}", player_entity_id.index().size_bytes());
    println!("size byte of entity_id = {}", player_entity_id.index().size_bytes());
    println!("size byte of entity_id = {}", player_entity_id.index().size_bytes());
    println!("size byte of entity_id = {}", player_entity_id.index().size_bytes());
    println!("size byte of login info = {}", login_info.size_bytes());
    drop(world);
    Packet::send_packet(0x2B, &login_info, &mut client.tcp_stream)?;
    synchro_position(client, _packet)
}

#[derive(Encode, MSerialize)]
struct PositionPlayer {
    x: f64,
    y: f64,
    z: f64,
    yaw: f32,
    pitch: f32,
    flags: u8,
    teleport_id: VarInt,
}

pub fn synchro_position(client: &mut Client, _packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    let new_pos = PositionPlayer {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        yaw: 0.0,
        pitch: 0.0,
        flags: 0,
        teleport_id: 42.into(),
    };
    Packet::send_packet(0x40, &new_pos, &mut client.tcp_stream)
}

pub fn confirm_teleportation(_client: &mut Client, packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    let teleport_id = leb128::read::signed(&mut packet.data)?;
    assert_eq!(teleport_id, 42);
    Ok(())
}