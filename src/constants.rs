#![allow(dead_code)]

pub const CHARACTER_SLOTS: usize = 10;
pub const USER_DATA_11: usize = 2_359_312; // 0x240010
pub const PROFILES: usize = CHARACTER_SLOTS;

pub const PC_OPTION_DATA: usize = 178; //0xB2

pub const MENU_SYSTEM_SAVE_LOAD: usize = 6_152; // 0x1808
pub const FACE_PRESETS: usize = 15;
pub const CS_MENU_SYSTEM_SAVE_LOAD_PADDING: usize = 1_576;
pub const CHARACTER_NAME: usize = 16;
pub const PROFILE_FACE_DATA: usize = 292; // 0x124
pub const PROFILE_UNK_PADDING: usize = 7;
pub const MAP_ID: usize = 4;

pub const PROFILE_EQUIPMENT_RAW: usize = 232; //0xE8

pub const FACE_PRESET_UNK0X00: usize = 20;
pub const FACE_PRESET_UNK0X6C: usize = 512;
pub const FACE_PRESET_PAD: usize = 10;

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

pub const SPEFFECT_ENTRY_COUNT: usize = 13;

// Inventory and InventoryItem in types.rs
pub const INVENTORY_HELD_COMMON_SLOT_COUNT: usize = 0xA80;
pub const INVENTORY_STORAGE_COMMON_SLOT_COUNT: usize = 0x780;
pub const INVENTORY_HELD_KEY_SLOT_COUNT: usize = 0x180;
pub const INVENTORY_STORAGE_KEY_SLOT_COUNT: usize = 0x80;

// EquippedSpells in types.rs
pub const EQUIPPED_SPELL_COUNT: usize = 14;

// EquippedItems in types.rs
pub const EQUIPPED_QUICK_ITEMS: usize = 10;
pub const EQUIPPED_POUCH_ITEMS: usize = 6;

// CharacterSlot in types.rs
pub const EQUIPPED_GESTURES: usize = 6;
pub const GESTURES: usize = 64;
