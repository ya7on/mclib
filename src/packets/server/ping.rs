use crate::packets::base::MCPacket;
use crate::types::base::MCType;
use crate::types::{MCLong, MCVarInt};
use mclib_macros::MCPacket;

#[derive(MCPacket, Debug, Clone)]
#[packet(packet_id = 0x01)]
pub struct PingRequest {
    pub payload: MCLong,
}
