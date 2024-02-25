use crate::packets::base::MCPacket;
use crate::types::base::MCType;
use crate::types::{MCFloat, MCPosition, MCVarInt};
use mclib_macros::MCPacket;

#[derive(MCPacket, Debug, Clone)]
#[packet(packet_id = 0x52)]
pub struct SetDefaultSpawnPosition {
    pub location: MCPosition,
    pub angle: MCFloat,
}
