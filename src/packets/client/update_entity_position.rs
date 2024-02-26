use crate::packets::packet_ids::current_version;
use crate::types::{MCBoolean, MCShort, MCVarInt};
use crate::{MCPacket, MCType};

#[derive(MCPacket, Debug, Clone)]
#[packet(packet_id = current_version::play::client::UPDATE_ENTITY_POSITION)]
pub struct UpdateEntityPosition {
    pub entity_id: MCVarInt,
    pub delta_x: MCShort,
    pub delta_y: MCShort,
    pub delta_z: MCShort,
    pub on_ground: MCBoolean,
}
