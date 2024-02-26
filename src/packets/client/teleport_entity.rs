use crate::packets::packet_ids::current_version;
use crate::types::{MCAngle, MCBoolean, MCDouble, MCVarInt};
use crate::{MCPacket, MCType};

#[derive(MCPacket, Debug, Clone)]
#[packet(packet_id = current_version::play::client::TELEPORT_ENTITY)]
pub struct TeleportEntity {
    pub entity_id: MCVarInt,
    pub x: MCDouble,
    pub y: MCDouble,
    pub z: MCDouble,
    pub yaw: MCAngle,
    pub pitch: MCAngle,
    pub on_ground: MCBoolean,
}
