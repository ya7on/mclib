use crate::packets::base::MCPacket;
use crate::types::base::MCType;
use crate::types::{MCByte, MCDouble, MCFloat, MCVarInt};
use mclib_macros::MCPacket;

#[derive(MCPacket, Debug, Clone)]
#[packet(packet_id = 0x3E)]
pub struct SynchronizePlayerPosition {
    pub x: MCDouble,
    pub y: MCDouble,
    pub z: MCDouble,
    pub yaw: MCFloat,
    pub pitch: MCFloat,
    pub flags: MCByte,
    pub teleport_id: MCVarInt,
}
