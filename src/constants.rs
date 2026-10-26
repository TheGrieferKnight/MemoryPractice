#![allow(dead_code)]

// EldenRingSave
pub const CHARACTER_SLOTS: usize = 10;
pub const MAGIC_SIZE: usize = 4;
pub const MAGIC_SL2: [u8; 4] = [83, 76, 50, 0]; // [0x53, 0x4C, 0x32, 0x00]
pub const MAGIC_PLAYSTATION: [u8; 4] = [203, 1, 156, 44]; // [0xCB, 0x01, 0x9C, 0x2C]
pub const MAGIC_BND4_HEX: [u8; 4] = [66, 78, 68, 52]; // [0x42, 0x4E, 0x44, 0x34]
pub const PC_HEADER_OFFSET: usize = 4; // 0x004
pub const PC_HEADER_SIZE: usize = 764; // 0x2FC
pub const PLAYSTATION_HEADER_OFFSET: usize = 4; // 0x004
pub const PLAYSTATION_HEADER_SIZE: usize = 108; // 0x6C

// SPEffect
pub const SPEFFECT_ENTRY_COUNT: usize = 13;

// MenuSystemSaveLoad
pub const MENU_SYSTEM_SAVE_LOAD: usize = 6_152; // 0x1808

// CSMenuSystemSaveLoad
pub const FACE_PRESETS: usize = 15;
pub const CS_MENU_SYSTEM_SAVE_LOAD_PADDING: usize = 1_576;

// ProfileSummary
pub const PROFILES: usize = CHARACTER_SLOTS;

// Profile
pub const CHARACTER_NAME: usize = 16;
pub const PROFILE_FACE_DATA: usize = 292; // 0x124
pub const PROFILE_UNK_PADDING: usize = 7;
pub const MAP_ID: usize = 4;

// ProfileEquipment
pub const PROFILE_EQUIPMENT_RAW: usize = 232; //0xE8

// FacePreset
pub const FACE_PRESET_UNK0X00: usize = 20;
pub const FACE_PRESET_UNK0X6C: usize = 512;
pub const FACE_PRESET_PAD: usize = 10;

// Inventory and InventoryItem
pub const INVENTORY_HELD_COMMON_SLOT_COUNT: usize = 2_688; // 0xA80
pub const INVENTORY_STORAGE_COMMON_SLOT_COUNT: usize = 1_920; // 0x780
pub const INVENTORY_HELD_KEY_SLOT_COUNT: usize = 384; // 0x180
pub const INVENTORY_STORAGE_KEY_SLOT_COUNT: usize = 128; // 0x80

// EquippedSpells
pub const EQUIPPED_SPELL_COUNT: usize = 14;

// EquippedItems
pub const EQUIPPED_QUICK_ITEMS: usize = 10;
pub const EQUIPPED_POUCH_ITEMS: usize = 6;

// CharacterSlot
pub const EQUIPPED_GESTURES: usize = 6;
pub const GESTURES: usize = 64;
pub const CHECKSUM_SIZE: usize = 16;

// UserDataX
pub const USER_DATA_X_UNK0X8: usize = 8;
pub const USER_DATA_X_UNK0X10: usize = 16;
pub const USER_DATA_X_SP_EFFECTS: usize = 13;
pub const USER_DATA_X_FACE_DATA: usize = 303;
pub const USER_DATA_X_EVENT_FLAGS: usize = 1_833_375;
pub const USER_DATA_X_GAITEM_MAP_HANDLE_TYPE_MASK: u32 = 4_026_531_840; // 0xF0000000
pub const USER_DATA_X_GAITEM_MAP_HANDLE_TYPE_C0000000: u32 = 3_221_225_472; // 0xC0000000
pub const USER_DATA_X_GAITEM_MAP_HANDLE_TYPE_80000000: u32 = 2_147_483_648; // 0x80000000
pub const USER_DATA_X_GAITEM_MAP_OLD_VERSION_MAX: u32 = 81;
pub const USER_DATA_X_TEMP_SPAWN_POINT_ENTITY_ID_MIN_VERSION: u32 = 65;
pub const USER_DATA_X_GAME_MAN_0XCB3_MIN_VERSION: u32 = 66;
pub const PC_CHARACTER_SLOT_SIZE: usize = 2_621_456; // 0x280010
pub const GAITEM_COUNT_OLD: usize = 5118;
pub const GAITEM_COUNT_NEW: usize = 5120;

// UserData10
pub const USER_DATA_10_MD5: u64 = 16;
pub const USER_DATA_10_VERSION: u64 = 4;
pub const USER_DATA_10_STEAM_ID: u64 = 8;
pub const USER_DATA_10_SETTINGS: u64 = 320; // 0x140
pub const USER_DATA_10_PROFILE_SUMMARY_PADDING: usize = 5;
pub const USER_DATA_10_KEY_CONFIG_SAVE_LOAD_PADDING: usize = 8;
pub const USER_DATA_10_SIZE: u64 = 393_232; // 0x60010
pub const PC_OPTION_DATA: usize = 178; //0xB2

// UserData11
pub const USER_DATA_11_DATA_PADDING: usize = 16;
pub const USER_DATA_11: usize = 2_359_312; // 0x240010

// PlayerGameDataHash
pub const PLAYER_GAME_DATA_HASH_REST: usize = 84; // 0x54

// DLC
pub const DLC_UNUSED: usize = 46;

// PS5Activity
pub const PS5_ACTIVITY_DATA: usize = 32; // 0x20

// NetMan
pub const NET_MAN_DATA: usize = 131_072; // 0x20000

// PlayerCoordinates
pub const PLAYER_COORDINATES_COORDINATES: usize = 3;
pub const PLAYER_COORDINATES_UNK_COORDINATES: usize = 3;
pub const PLAYER_COORDINATES_ANGLE: usize = 4;
pub const PLAYER_COORDINATES_UNK_ANGLE: usize = 4;

// RendMan
pub const REND_MAN_DATA_SIZE_MAX: u32 = 1_048_576; // 0x100000

// WorldGeomMan
pub const WORLD_GEOM_MAN_DATA_SIZE_MAX: u32 = 1_048_576; // 0x100000

// WorldArea
pub const WORLD_AREA_DATA_SIZE_MAX: u32 = 65_536; // 0x10000

// FieldArea
pub const FIELD_AREA_DATA_SIZE_MAX: u32 = 65_536; // 0x10000

// TutorialData
pub const TUTORIAL_DATA_SIZE_MAX: u32 = 65_536; // 0x10000
pub const TUTORIAL_DATA_SIZE_DEFAULT: u32 = 1_024; // 0x400

// TutorialDataChunk
pub const TUTORIAL_DATA_CHUNK_COUNT: u32 = 4;
pub const TUTORIAL_DATA_CHUNK_TUTORIAL_IDS: u32 = 4;

// GaitemGameData
pub const GAITEM_GAME_DATA_ENTRIES: usize = 14000;

// GaitemGameDataEntry
pub const GAITEM_GAME_DATA_ENTRY_PADDING: usize = 3;

// MenuSaveLoad
pub const MENU_SAVE_LOAD_DATA_SIZE_MAX: u32 = 65_536; // 0x10000
pub const MENU_SAVE_LOAD_DATA_SIZE_DEFAULT: u32 = 4_096; // 0x1000

// BloodStain
pub const BLOOD_STAIN_COORDINATES: usize = 3;
pub const BLOOD_STAIN_ANGLE: usize = 4;

// RideGameData
pub const RIDE_GAME_DATA_COORDINATES: usize = 3;
pub const RIDE_GAME_DATA_ANGLE: usize = 4;

// PlayerGameData
pub const PLAYER_GAME_DATA_UNK0XC0: usize = 24;
pub const PLAYER_GAME_DATA_UNK0XDD: usize = 26;
pub const PLAYER_GAME_DATA_UNK0XFB: usize = 21;
pub const PLAYER_GAME_DATA_UNK0X17C: usize = 52;
pub const PLAYER_GAME_DATA_CHARACTER_NAME: usize = 16;
pub const PLAYER_GAME_DATA_PASSWORD: usize = 8;
pub const PLAYER_GAME_DATA_GROUP_PASSWORD1: usize = 8;
pub const PLAYER_GAME_DATA_GROUP_PASSWORD2: usize = 8;
pub const PLAYER_GAME_DATA_GROUP_PASSWORD3: usize = 8;
pub const PLAYER_GAME_DATA_GROUP_PASSWORD4: usize = 8;
pub const PLAYER_GAME_DATA_GROUP_PASSWORD5: usize = 8;

// KeyConfigSaveLoad
pub const KEY_CONFIG_SAVE_LOAD_SIZE_PADDING: usize = 4;
pub const KEY_CONFIG_SAVE_LOAD_DATA_SIZE_MAX: u32 = 1_048_576; // 0x100000
