use crate::packets::base::MCPacket;
use crate::packets::packet_ids::current_version;
use crate::types::base::MCType;
use crate::types::{MCString, MCUuid, MCVarInt};
use mclib_macros::MCPacket;

#[derive(MCPacket, Debug, Clone)]
#[packet(packet_id = current_version::login::server::LOGIN_START)]
pub struct LoginStart {
    pub name: MCString,
    pub player_uuid: MCUuid,
}
