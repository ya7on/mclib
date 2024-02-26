mod version_1_20_4 {
    pub mod handshaking {
        pub mod server {
            pub const HANDSHAKE: i32 = 0x00;
        }
    }
    pub mod status {
        pub const PING: i32 = 0x01;
        pub mod client {
            pub const STATUS_RESPONSE: i32 = 0x00;
        }
        pub mod server {
            pub const STATUS_REQUEST: i32 = 0x00;
        }
    }
    pub mod login {
        pub mod client {
            pub const LOGIN_SUCCESS: i32 = 0x02;
        }
        pub mod server {
            pub const LOGIN_START: i32 = 0x00;
            pub const LOGIN_ACKNOWLEDGED: i32 = 0x03;
        }
    }
    pub mod configuration {
        pub mod client {
            pub const FINISH_CONFIGURATION: i32 = 0x02;
            pub const REGISTRY_DATA: i32 = 0x05;
        }
        pub mod server {
            pub const FINISH_CONFIGURATION: i32 = 0x02;
        }
    }
    pub mod play {
        pub mod client {
            pub const SPAWN_ENTITY: i32 = 0x01;
            pub const KEEP_ALIVE: i32 = 0x24;
            pub const CHUNK_DATA_AND_UPDATE_LIGHT: i32 = 0x25;
            pub const PLAY: i32 = 0x29;
            pub const PLAYER_ABILITIES: i32 = 0x36;
            pub const SET_DEFAULT_SPAWN_POSITION: i32 = 0x54;
            pub const SYNCHRONIZE_PLAYER_POSITION: i32 = 0x3E;
            pub const TELEPORT_ENTITY: i32 = 0x6D;
        }
        pub mod server {
            pub const KEEP_ALIVE: i32 = 0x15;
            pub const SET_PLAYER_POSITION: i32 = 0x17;
        }
    }
}

pub mod current_version {
    pub use super::version_1_20_4::*;
}
