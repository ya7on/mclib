use crate::packets::base::MCPacket;
use crate::packets::packet_ids::current_version;
use crate::types::base::MCType;
use crate::types::{MCString, MCUShort, MCVarInt};
use mclib_macros::MCPacket;

#[derive(Debug, Clone)]
pub enum HandshakeNextState {
    Status = 0x01,
    Login = 0x02,
    Unknown,
}

impl From<i32> for HandshakeNextState {
    fn from(value: i32) -> Self {
        match value {
            0x01 => Self::Status,
            0x02 => Self::Login,
            _ => Self::Unknown,
        }
    }
}

#[derive(MCPacket, Debug, Clone)]
#[packet(packet_id = current_version::handshaking::server::HANDSHAKE)]
pub struct Handshake {
    pub protocol_version: MCVarInt,
    pub server_address: MCString,
    pub server_port: MCUShort,
    pub next_state: MCVarInt,
}
