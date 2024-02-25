use crate::packets::base::MCPacket;
use crate::types::base::MCType;
use crate::types::{MCLong, MCVarInt};
use mclib_macros::MCPacket;

#[derive(MCPacket, Debug, Clone)]
#[packet(packet_id = 0x14)]
pub struct ServerboundKeelAlivePlay {
    pub keepalive_id: MCLong,
}
