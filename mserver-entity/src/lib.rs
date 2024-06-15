use bevy_ecs::component::Component;
use uuid::Uuid;

use mserver_types::varint::VarInt;

#[derive(Component)]
pub struct MEntity {
    pub uuid: Uuid,
    pub r#type: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: u8,
    pub yaw: u8,
    pub head_yaw: u8,
    pub data: VarInt,
    pub vx: u16,
    pub vy: u16,
    pub vz: u16,
}

