#![allow(dead_code)]

pub const PC_CHARACTER_SLOT_SIZE: usize = 0x280010;

pub const MAGIC_SIZE: usize = 4;

pub const MAGIC_SL2: [u8; 4] = [0x53, 0x4C, 0x32, 0x00];
pub const MAGIC_PLAYSTATION: [u8; 4] = [0xCB, 0x01, 0x9C, 0x2C];
pub const MAGIC_BND4_HEX: [u8; 4] = [0x42, 0x4E, 0x44, 0x34];

pub const PC_HEADER_OFFSET: usize = 0x004;
pub const PC_HEADER_SIZE: usize = 0x2FC;

pub const PLAYSTATION_HEADER_OFFSET: usize = 0x004;
pub const PLAYSTATION_HEADER_SIZE: usize = 0x6C;

pub const CHECKSUM_SIZE: usize = 16;

pub const GAITEM_COUNT_OLD: usize = 5118;
pub const GAITEM_COUNT_NEW: usize = 5120;