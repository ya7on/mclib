use crate::packets::base::MCPacket;
use crate::packets::packet_ids::current_version;
use crate::types::base::MCType;
use crate::types::{MCBoolean, MCByte, MCInt, MCLong, MCPosition, MCString, MCUByte, MCVarInt};
use mclib_macros::{MCPacket, MCType};
use std::io::Read;

#[derive(MCType, Debug, Clone)]
pub struct DeathInfo {
    pub death_dimension_name: MCString,
    pub death_location: MCPosition,
}

#[derive(MCPacket, Debug, Clone)]
#[packet(packet_id = current_version::play::client::PLAY)]
pub struct Play {
    pub entity_id: MCInt,
    pub is_hardcore: MCBoolean,
    pub dimensions: Vec<MCString>,
    pub max_players: MCVarInt,
    pub view_distance: MCVarInt,
    pub simulation_distance: MCVarInt,
    pub reduced_debug_info: MCBoolean,
    pub enable_respawn_screen: MCBoolean,
    pub do_limited_crafting: MCBoolean,
    pub dimension_type: MCString,
    pub dimension_name: MCString,
    pub hashed_seed: MCLong,
    pub game_mode: MCUByte,
    pub previous_game_mode: MCByte,
    pub is_debug: MCBoolean,
    pub is_flat: MCBoolean,
    pub death_info: Option<DeathInfo>,
    pub portal_cooldown: MCVarInt,
}
