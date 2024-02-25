use crate::packets::base::MCPacket;
use crate::types::base::MCType;
use crate::types::{MCString, MCUuid, MCVarInt};
use mclib_macros::{MCPacket, MCType};
use std::io::Read;

#[derive(MCType, Debug, Clone)]
pub struct LoginSuccessProperty {
    pub name: MCString,
    pub value: MCString,
    pub signature: Option<MCString>,
}

#[derive(MCPacket, Debug, Clone)]
#[packet(packet_id = 0x02)]
pub struct LoginSuccess {
    pub uuid: MCUuid,
    pub username: MCString,
    pub properties: Vec<LoginSuccessProperty>,
}
