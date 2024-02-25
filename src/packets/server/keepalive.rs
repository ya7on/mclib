use crate::packets::base::MCPacket;
use crate::packets::packet_ids::current_version;
use crate::types::base::MCType;
use crate::types::{MCLong, MCVarInt};
use mclib_macros::MCPacket;

#[derive(MCPacket, Debug, Clone)]
#[packet(packet_id = current_version::play::server::KEEP_ALIVE)]
pub struct ServerboundKeelAlivePlay {
    pub keepalive_id: MCLong,
}
