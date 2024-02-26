use crate::packets::packet_ids::current_version;
use crate::types::{MCAngle, MCShort};
use crate::types::{MCDouble, MCUuid, MCVarInt};
use crate::{MCPacket, MCType};

pub enum EntityType {
    Wolf = 118,
    Zombie = 120,
}

#[derive(MCPacket, Clone, Debug)]
#[packet(packet_id = current_version::play::client::SPAWN_ENTITY)]
pub struct SpawnEntity {
    pub entity_id: MCVarInt,
    pub entity_uuid: MCUuid,
    pub entity_type: MCVarInt,
    pub x: MCDouble,
    pub y: MCDouble,
    pub z: MCDouble,
    pub pitch: MCAngle,
    pub yaw: MCAngle,
    pub head_yaw: MCAngle,
    pub data: MCVarInt,
    pub velocity_x: MCShort,
    pub velocity_y: MCShort,
    pub velocity_z: MCShort,
}
