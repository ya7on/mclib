use crate::packets::base::MCPacket;
use crate::packets::packet_ids::current_version;
use crate::types::base::MCType;
use crate::types::{MCByte, MCDouble, MCFloat, MCVarInt};
use mclib_macros::MCPacket;

#[derive(MCPacket, Debug, Clone)]
#[packet(packet_id = current_version::play::client::SYNCHRONIZE_PLAYER_POSITION)]
pub struct SynchronizePlayerPosition {
    pub x: MCDouble,
    pub y: MCDouble,
    pub z: MCDouble,
    pub yaw: MCFloat,
    pub pitch: MCFloat,
    pub flags: MCByte,
    pub teleport_id: MCVarInt,
}
