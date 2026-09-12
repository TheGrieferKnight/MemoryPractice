#![allow(dead_code)]

use std::io::{self, Read};

use crate::cursor::CursorReader;

#[derive(Debug, Default)]
pub struct Gaitem {
    pub gaitem_handle: u32,
    pub item_id: u32,
    pub unk_0x10: Option<u32>,
    pub unk_0x14: Option<u32>,
    pub gem_gaitem_handle: Option<u32>,
    pub unk_0x1c: Option<u8>,
}

#[derive(Debug)]
pub struct PlayerGameData {
    pub unk0x0: u32,
    pub unk0x4: u32,
    pub hp: u32,
    pub max_hp: u32,
    pub base_max_hp: u32,
    pub fp: u32,
    pub max_fp: u32,
    pub base_max_fp: u32,
    pub unk0x20: u32,
    pub sp: u32, // (stamina)
    pub max_sp: u32,
    pub base_max_sp: u32,
    pub unk0x30: u32,
    pub vigor: u32,
    pub mind: u32,
    pub endurance: u32,
    pub strength: u32,
    pub dexterity: u32,
    pub intelligence: u32,
    pub faith: u32,
    pub arcane: u32,
    pub unk0x54: u32,
    pub unk0x58: u32,
    pub unk0x5c: u32,
    pub level: u32,
    pub runes: u32,
    pub runes_memory: u32,
    pub unk0x6c: u32,
    pub poison_buildup: u32,
    pub rot_buildup: u32,
    pub bleed_buildup: u32,
    pub death_buildup: u32,
    pub frost_buildup: u32,
    pub sleep_buildup: u32,
    pub madness_buildup: u32,
    pub unk0x8c: u32,
    pub unk0x90: u32,
    pub character_name: String, //(UTF-16LE, 16 chars max)
    pub terminator: u16,
    pub gender: u8,
    pub archetype: u8,
    pub unk0xb8: u8,
    pub unk0xb9: u8,
    pub voice_type: u8,
    pub gift: u8,
    pub unk0xbc: u8,
    pub unk0xbd: u8,
    pub additional_talisman_slot_count: u8,
    pub summon_spirit_level: u8,
    pub unk0xc0: [u8; 24],
    pub furl_calling_finger_on: u8, // (bool)
    pub unk0xd9: u8,
    pub matchmaking_weapon_level: u8,
    pub white_cipher_ring_on: u8, //(bool)
    pub blue_cipher_ring_on: u8,  // (bool)
    pub unk0xdd: [u8; 26],
    pub great_rune_on: u8, // (bool)
    pub unk0xf8: u8,
    pub max_crimson_flask_count: u8,
    pub max_cerulean_flask_count: u8,
    pub unk0xfb: [u8; 21],
    pub password: String, // (UTF-16LE, 8 chars + terminator)
    pub password_terminator: u16,
    pub group_password1: String,
    pub group_password1_terminator: u16,
    pub group_password2: String,
    pub group_password2_terminator: u16,
    pub group_password3: String,
    pub group_password3_terminator: u16,
    pub group_password4: String,
    pub group_password4_terminator: u16,
    pub group_password5: String,
    pub group_password5_terminator: u16,
    pub unk0x17c: [u8; 52], // (padding)
}

impl PlayerGameData {
    pub fn read_unk0xc0<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<[u8; 24]> {
        let mut unk0xc0 = [0u8; 24];

        cursor.read_exact(&mut unk0xc0)?;

        Ok(unk0xc0)
    }
    pub fn read_unk0xdd<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<[u8; 26]> {
        let mut unk0xdd = [0u8; 26];

        cursor.read_exact(&mut unk0xdd)?;

        Ok(unk0xdd)
    }
    pub fn read_unk0xfb<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<[u8; 21]> {
        let mut unk0xfb = [0u8; 21];

        cursor.read_exact(&mut unk0xfb)?;

        Ok(unk0xfb)
    }
    pub fn read_unk0x17c<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<[u8; 52]> {
        let mut unk0x17c = [0u8; 52];

        cursor.read_exact(&mut unk0x17c)?;

        Ok(unk0x17c)
    }

    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            unk0x0: cursor.read_u32_le()?,
            unk0x4: cursor.read_u32_le()?,
            hp: cursor.read_u32_le()?,
            max_hp: cursor.read_u32_le()?,
            base_max_hp: cursor.read_u32_le()?,
            fp: cursor.read_u32_le()?,
            max_fp: cursor.read_u32_le()?,
            base_max_fp: cursor.read_u32_le()?,
            unk0x20: cursor.read_u32_le()?,
            sp: cursor.read_u32_le()?, // (stamina)
            max_sp: cursor.read_u32_le()?,
            base_max_sp: cursor.read_u32_le()?,
            unk0x30: cursor.read_u32_le()?,
            vigor: cursor.read_u32_le()?,
            mind: cursor.read_u32_le()?,
            endurance: cursor.read_u32_le()?,
            strength: cursor.read_u32_le()?,
            dexterity: cursor.read_u32_le()?,
            intelligence: cursor.read_u32_le()?,
            faith: cursor.read_u32_le()?,
            arcane: cursor.read_u32_le()?,
            unk0x54: cursor.read_u32_le()?,
            unk0x58: cursor.read_u32_le()?,
            unk0x5c: cursor.read_u32_le()?,
            level: cursor.read_u32_le()?,
            runes: cursor.read_u32_le()?,
            runes_memory: cursor.read_u32_le()?,
            unk0x6c: cursor.read_u32_le()?,
            poison_buildup: cursor.read_u32_le()?,
            rot_buildup: cursor.read_u32_le()?,
            bleed_buildup: cursor.read_u32_le()?,
            death_buildup: cursor.read_u32_le()?,
            frost_buildup: cursor.read_u32_le()?,
            sleep_buildup: cursor.read_u32_le()?,
            madness_buildup: cursor.read_u32_le()?,
            unk0x8c: cursor.read_u32_le()?,
            unk0x90: cursor.read_u32_le()?,
            character_name: cursor.read_utf16_le_to_string(16)?, //(UTF-16LE, 16 chars max)
            terminator: cursor.read_u16_le()?,
            gender: cursor.read_u8_le()?,
            archetype: cursor.read_u8_le()?,
            unk0xb8: cursor.read_u8_le()?,
            unk0xb9: cursor.read_u8_le()?,
            voice_type: cursor.read_u8_le()?,
            gift: cursor.read_u8_le()?,
            unk0xbc: cursor.read_u8_le()?,
            unk0xbd: cursor.read_u8_le()?,
            additional_talisman_slot_count: cursor.read_u8_le()?,
            summon_spirit_level: cursor.read_u8_le()?,
            unk0xc0: Self::read_unk0xc0(cursor)?,
            furl_calling_finger_on: cursor.read_u8_le()?, // (bool)
            unk0xd9: cursor.read_u8_le()?,
            matchmaking_weapon_level: cursor.read_u8_le()?,
            white_cipher_ring_on: cursor.read_u8_le()?, //(bool)
            blue_cipher_ring_on: cursor.read_u8_le()?,  // (bool)
            unk0xdd: Self::read_unk0xdd(cursor)?,
            great_rune_on: cursor.read_u8_le()?, // (bool)
            unk0xf8: cursor.read_u8_le()?,
            max_crimson_flask_count: cursor.read_u8_le()?,
            max_cerulean_flask_count: cursor.read_u8_le()?,
            unk0xfb: Self::read_unk0xfb(cursor)?,
            password: cursor.read_utf16_le_to_string(8)?, // (UTF-16LE, 8 chars + terminator)
            password_terminator: cursor.read_u16_le()?,
            group_password1: cursor.read_utf16_le_to_string(8)?,
            group_password1_terminator: cursor.read_u16_le()?,
            group_password2: cursor.read_utf16_le_to_string(8)?,
            group_password2_terminator: cursor.read_u16_le()?,
            group_password3: cursor.read_utf16_le_to_string(8)?,
            group_password3_terminator: cursor.read_u16_le()?,
            group_password4: cursor.read_utf16_le_to_string(8)?,
            group_password4_terminator: cursor.read_u16_le()?,
            group_password5: cursor.read_utf16_le_to_string(8)?,
            group_password5_terminator: cursor.read_u16_le()?,
            unk0x17c: Self::read_unk0x17c(cursor)?, // (padding)
        })
    }
}