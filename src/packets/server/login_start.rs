use crate::packets::base::MCPacket;
use crate::types::base::MCType;
use crate::types::{MCString, MCUuid, MCVarInt};
use mclib_macros::MCPacket;

#[derive(MCPacket, Debug, Clone)]
#[packet(packet_id = 0x00)]
pub struct LoginStart {
    pub name: MCString,
    pub player_uuid: MCUuid,
}
