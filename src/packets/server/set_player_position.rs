use crate::packets::base::MCPacket;
use crate::types::base::MCType;
use crate::types::{MCBoolean, MCDouble, MCVarInt};
use mclib_macros::MCPacket;

#[derive(MCPacket, Debug, Clone)]
#[packet(packet_id = 0x16)]
pub struct SetPlayerPosition {
    pub x: MCDouble,
    pub feet_y: MCDouble,
    pub z: MCDouble,
    pub on_ground: MCBoolean,
}
