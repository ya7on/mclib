use crate::packets::base::MCPacket;
use crate::packets::packet_ids::current_version;
use crate::types::base::MCType;
use crate::types::{MCString, MCVarInt};
use mclib_macros::MCPacket;

#[derive(MCPacket, Debug, Clone)]
#[packet(packet_id = current_version::status::client::STATUS_RESPONSE)]
pub struct StatusResponse {
    pub json_response: MCString,
}
