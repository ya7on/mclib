use crate::packets::base::MCPacket;
use crate::packets::packet_ids::current_version;
use crate::types::base::MCType;
use crate::types::{MCByte, MCFloat, MCVarInt};
use mclib_macros::MCPacket;

#[derive(MCPacket, Debug, Clone)]
#[packet(packet_id = current_version::play::client::PLAYER_ABILITIES)]
pub struct PlayerAbilities {
    pub flags: MCByte,
    pub flying_speed: MCFloat,
    pub field_of_view_modifier: MCFloat,
}
