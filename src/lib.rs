//! Minecraft Java Edition protocol helpers
//!
//! Current version of crate supports __765__ protocol version, Minecraft version __1.20.4__
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

pub mod chunk_format;
pub mod nbt;
pub mod packets;
pub mod types;
pub mod utils;

pub use crate::packets::base::MCPacket;
pub use crate::types::base::MCType;
pub use mclib_macros::MCPacket;
pub use mclib_macros::MCType;
