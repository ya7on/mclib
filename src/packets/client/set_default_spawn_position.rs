use crate::packets::base::MCPacket;
use crate::packets::packet_ids::current_version;
use crate::types::base::MCType;
use crate::types::{MCFloat, MCPosition, MCVarInt};
use mclib_macros::MCPacket;

#[derive(MCPacket, Debug, Clone)]
#[packet(packet_id = current_version::play::client::SET_DEFAULT_SPAWN_POSITION)]
pub struct SetDefaultSpawnPosition {
    pub location: MCPosition,
    pub angle: MCFloat,
}
