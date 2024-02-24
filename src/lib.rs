//! Minecraft Java Edition protocol helpers
//!
//! Current version of crate supports __763__ protocol version, Minecraft version __1.20.2__
//!
//! # Examples
//!
//! - [Writing to packet](#writing-to-packet)
//! - [Reading from binary data](#reading-from-binary-data)
//! - [Writing new packet](#writing-new-packet)
//! - [NBT](#nbt)
//!
//! # Writing to packet
//!
//! ```
//! use mclib::MCPacket;
//! use mclib::packets::server::PingRequest;
//! use mclib::types::MCLong;
//!
//! let packet = PingRequest {
//!     payload: MCLong::from(1337),
//! };
//!
//! // Struct will be packed into Minecraft protocol format with packet id prefixed
//! let binary: Vec<u8> = packet.pack();
//! ```
//!
//! # Reading from binary data
//! ```no_run
//! use mclib::MCPacket;
//! use mclib::packets::server::LoginStart;
//!
//! // You can use any data type that implements std::io::Read
//! let raw_data: Vec<u8> = vec![0x01, 0x03, 0x03, 0x07];
//! let mut binary_data = std::io::Cursor::new(raw_data);
//!
//! let login_start = LoginStart::unpack(&mut binary_data);
//! ```
//!
//! # Writing new packet
//! If your want to write new packet, you need to create structure with derive [MCPacket] trait and
//! with Minecraft data types: you can use existing [types] or create new by implementing [MCType] trait.
//! Derived trait will automatically add methods like `pack()` and `unpack()`
//!
//! ```
//! use mclib::types::{MCShort, MCVarInt};
//! use mclib::{MCPacket, MCType};
//!
//! #[derive(MCPacket, Debug, Clone)]
//! #[packet(packet_id = 0x00)] // Specify packet_id here
//! pub struct NewMinecraftPacket {
//!     field_1: MCShort
//! }
//! ```
//!
//! # NBT
//! ```nbt
//! TAG_Compound('hello world'): 1 entry
//!   {
//!     TAG_String('name'): 'Bananrama'
//!   }
//! ```
//! Code equivalent for this NBT will be
//!
//! ```
//! use mclib::nbt::{NBT, IntoNBTTag};
//!
//! let nbt = NBT(
//!     Some("hello world".to_string()),
//!     vec![("name", "Bananrama".to_nbt())].to_nbt(),
//! );
//! ```
//! For more information about NBT tags [their pages](nbt)

pub use mclib_macros::MCPacket;
pub use mclib_macros::MCType;
pub use mclib_protocol::packets::base::MCPacket;
pub use mclib_protocol::types::base::MCType;

pub mod packets {
    //! Minecraft protocol packets
    pub mod server {
        //! Packets that bound to server (from client to server)
        pub use mclib_protocol::packets::server::finish_configuration::FinishConfigurationServerbound;
        pub use mclib_protocol::packets::server::handshake::{Handshake, HandshakeNextState};
        pub use mclib_protocol::packets::server::keepalive::ServerboundKeelAlivePlay;
        pub use mclib_protocol::packets::server::login_acknowledged::LoginAcknowledged;
        pub use mclib_protocol::packets::server::login_start::LoginStart;
        pub use mclib_protocol::packets::server::ping::PingRequest;
        pub use mclib_protocol::packets::server::set_player_position::SetPlayerPosition;
        pub use mclib_protocol::packets::server::status_request::StatusRequest;
    }
    pub mod client {
        //! Packets that bound to client (from server to client)
        pub use mclib_protocol::packets::client::chunk_data_and_update_light::ChunkDataAndUpdateLight;
        pub use mclib_protocol::packets::client::finish_configuration::FinishConfigurationClientbound;
        pub use mclib_protocol::packets::client::keepalive::ClientboundKeelAlivePlay;
        pub use mclib_protocol::packets::client::login_success::{
            LoginSuccess, LoginSuccessProperty,
        };
        pub use mclib_protocol::packets::client::play::{DeathInfo, Play};
        pub use mclib_protocol::packets::client::registry_data::RegistryData;
        pub use mclib_protocol::packets::client::set_default_spawn_position::SetDefaultSpawnPosition;
        pub use mclib_protocol::packets::client::status_response::StatusResponse;
        pub use mclib_protocol::packets::client::synchronize_player_position::SynchronizePlayerPosition;
    }
}
pub mod types {
    //! Minecraft data types
    pub use mclib_protocol::types::bitset::MCBitSet;
    pub use mclib_protocol::types::boolean::MCBoolean;
    pub use mclib_protocol::types::byte::MCByte;
    pub use mclib_protocol::types::byte_array::MCByteArray;
    pub use mclib_protocol::types::double::MCDouble;
    pub use mclib_protocol::types::float::MCFloat;
    pub use mclib_protocol::types::int::MCInt;
    pub use mclib_protocol::types::long::MCLong;
    pub use mclib_protocol::types::nbt::MCNBT;
    pub use mclib_protocol::types::position::MCPosition;
    pub use mclib_protocol::types::short::MCShort;
    pub use mclib_protocol::types::string::MCString;
    pub use mclib_protocol::types::ubyte::MCUByte;
    pub use mclib_protocol::types::ushort::MCUShort;
    pub use mclib_protocol::types::uuid::MCUuid;
    pub use mclib_protocol::types::varint::MCVarInt;
}
pub mod nbt {
    //! Named Binary Tag (NBT) protocol
    //!
    //! [More information](https://wiki.vg/NBT)
    pub use mclib_protocol::nbt::tags::base::{IntoNBTTag, NBTTag};
    /// TAG_Byte
    pub use mclib_protocol::nbt::tags::byte::TagByte;
    /// TAG_Byte_Array
    pub use mclib_protocol::nbt::tags::byte_array::TagByteArray;
    /// TAG_Compound (any non root)
    pub use mclib_protocol::nbt::tags::compound::TagCompound;
    /// TAG_Double
    pub use mclib_protocol::nbt::tags::double::TagDouble;
    /// TAG_Float
    pub use mclib_protocol::nbt::tags::float::TagFloat;
    /// TAG_Int
    pub use mclib_protocol::nbt::tags::int::TagInt;
    /// TAG_List
    pub use mclib_protocol::nbt::tags::list::TagList;
    /// TAG_Long
    pub use mclib_protocol::nbt::tags::long::TagLong;
    /// TAG_Long_Array
    pub use mclib_protocol::nbt::tags::long_array::TagLongArray;
    /// TAG_Short
    pub use mclib_protocol::nbt::tags::short::TagShort;
    /// TAG_String
    pub use mclib_protocol::nbt::tags::string::TagString;
    /// Parent NBT tag (Root TAG_Compound)
    pub use mclib_protocol::nbt::NBT;
}
pub mod chunk_format {
    pub use mclib_protocol::chunk_format::{
        data_array::DataArray, palleted_container::PalletedContainer, section::ChunkSection,
        ChunkData,
    };
}
