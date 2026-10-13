#![allow(dead_code)]

use std::io::{self, Read};

use crate::{constants::*, cursor::CursorReader};

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
    fn read_unk0xc0<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<[u8; 24]> {
        let mut unk0xc0 = [0u8; 24];

        cursor.read_exact(&mut unk0xc0)?;

        Ok(unk0xc0)
    }
    fn read_unk0xdd<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<[u8; 26]> {
        let mut unk0xdd = [0u8; 26];

        cursor.read_exact(&mut unk0xdd)?;

        Ok(unk0xdd)
    }
    fn read_unk0xfb<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<[u8; 21]> {
        let mut unk0xfb = [0u8; 21];

        cursor.read_exact(&mut unk0xfb)?;

        Ok(unk0xfb)
    }
    fn read_unk0x17c<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<[u8; 52]> {
        let mut unk0x17c = [0u8; 52];

        cursor.read_exact(&mut unk0x17c)?;

        Ok(unk0x17c)
    }

    /// Creates an instance of the PlayerGameData struct by reading the next 432 bytes from the Cursors current offset.
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

#[derive(Debug)]
pub struct SPEffect {
    pub sp_effect_id: u32,
    pub remaining_time: f32,
    pub unk0x8: u32,
    pub unk0x10: u32,
}

impl SPEffect {
    /// Creates an instance of the SPEffect struct by reading the next 16 bytes from the Cursors current offset.
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            sp_effect_id: cursor.read_u32_le()?,
            remaining_time: cursor.read_f32_le()?,
            unk0x8: cursor.read_u32_le()?,
            unk0x10: cursor.read_u32_le()?,
        })
    }
}

#[derive(Debug)]
pub struct EquipmentSlots {
    pub left_hand_armament1: u32,
    pub right_hand_armament1: u32,
    pub left_hand_armament2: u32,
    pub right_hand_armament2: u32,
    pub left_hand_armament3: u32,
    pub right_hand_armament3: u32,
    pub arrows1: u32,
    pub bolts1: u32,
    pub arrows2: u32,
    pub bolts2: u32,
    pub unk0x28: u32,
    pub unk0x2c: u32,
    pub head: u32,
    pub chest: u32,
    pub arms: u32,
    pub legs: u32,
    pub unk0x40: u32,
    pub talisman1: u32,
    pub talisman2: u32,
    pub talisman3: u32,
    pub talisman4: u32,
    pub unk0x54: u32,
}

impl EquipmentSlots {
    /// Creates an instance of the EquipmentSlots struct by reading the next 88 bytes from the Cursors current offset.
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            left_hand_armament1: cursor.read_u32_le()?,
            right_hand_armament1: cursor.read_u32_le()?,
            left_hand_armament2: cursor.read_u32_le()?,
            right_hand_armament2: cursor.read_u32_le()?,
            left_hand_armament3: cursor.read_u32_le()?,
            right_hand_armament3: cursor.read_u32_le()?,
            arrows1: cursor.read_u32_le()?,
            bolts1: cursor.read_u32_le()?,
            arrows2: cursor.read_u32_le()?,
            bolts2: cursor.read_u32_le()?,
            unk0x28: cursor.read_u32_le()?,
            unk0x2c: cursor.read_u32_le()?,
            head: cursor.read_u32_le()?,
            chest: cursor.read_u32_le()?,
            arms: cursor.read_u32_le()?,
            legs: cursor.read_u32_le()?,
            unk0x40: cursor.read_u32_le()?,
            talisman1: cursor.read_u32_le()?,
            talisman2: cursor.read_u32_le()?,
            talisman3: cursor.read_u32_le()?,
            talisman4: cursor.read_u32_le()?,
            unk0x54: cursor.read_u32_le()?,
        })
    }
}

#[derive(Debug)]
pub struct ActiveWeaponSlotsAndArmStyle {
    pub arm_style: u32,
    pub left_hand_weapon_active_slot: u32,
    pub right_hand_weapon_active_slot: u32,
    pub left_arrow_active_slot: u32,
    pub right_arrow_active_slot: u32,
    pub left_bolt_active_slot: u32,
    pub right_bolt_active_slot: u32,
}

impl ActiveWeaponSlotsAndArmStyle {
    /// Creates an instance of the ActiveWeaponSlotsAndArmStyle struct by reading the next 88 bytes from the Cursors current offset.
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            arm_style: cursor.read_u32_le()?,
            left_hand_weapon_active_slot: cursor.read_u32_le()?,
            right_hand_weapon_active_slot: cursor.read_u32_le()?,
            left_arrow_active_slot: cursor.read_u32_le()?,
            right_arrow_active_slot: cursor.read_u32_le()?,
            left_bolt_active_slot: cursor.read_u32_le()?,
            right_bolt_active_slot: cursor.read_u32_le()?,
        })
    }
}

#[derive(Debug)]
pub struct CharacterSlot {
    version: u32,
    map_id: u32,
    unk0x8: [u8; 8],
    unk0x10: [u8; 16],
    gaitem_map: Vec<Gaitem>,
    pub(crate) player_game_data: PlayerGameData,
    sp_effects: Box<[SPEffect; 13]>,
    equipped_items_equip_index: EquipmentSlots,
    active_weapon_slots_and_arm: ActiveWeaponSlotsAndArmStyle,
    equipped_items_item_id: EquipmentSlots,
    equipped_items_gaitem_handle: EquipmentSlots,
    inventory_held: Inventory,
    equipped_spells: EquippedSpells,
    equipped_items: EquippedItems,
    equipped_gestures: Vec<u32>,
    acquired_projectiles: AcquiredProjectiles,
    equipped_armaments_and_items: EquippedArmamentsAndItems,
    equipped_physics: EquippedPhysics,
    face_data: [u8; 303],
    inventory_storage_box: Inventory,
    gestures: Vec<u32>,
    unlocked_regions: Regions,
    horse: RideGameData,
    control_byte_maybe: u8,
    blood_stain: BloodStain,
    unk_gamedataman_0x120: u32,
    unk_gamedataman_0x88: u32,
    menu_profile_save_load: MenuSaveLoad,
    trophy_equip_data: TrophyEquipData,
    gaitem_game_data: GaitemGameData,
    tutorial_data: TutorialData,
    gameman_0x8c: u8,
    gameman_0x8d: u8,
    gameman_0x8e: u8,
    total_deaths_count: u32,
    character_type: i32,
    in_online_session_flag: u8,
    character_type_online: u32,
    last_rested_grace: u32,
    not_alone_flag: u8,
    in_game_countdown_timer: u32,
    unk_gamedataman_0x124: u32,
    event_flags: Vec<u8>, // 1_833_375
    event_flags_terminator: u8,
    field_area: FieldArea,
    world_area: WorldArea,
    world_geom_man: WorldGeomMan,
    world_geom_man2: WorldGeomMan,
    rend_man: RendMan,
    player_coordinates: PlayerCoordinates,
    game_man_0x5be: u8,
    game_man_0x5bf: u8,
    spawn_point_entity_id: u32,
    game_man_0xb64: u32,
    temp_spawn_point_entity_id: Option<u32>,
    game_man_0xcb3: Option<u8>,
    net_man: NetMan,
    world_area_weather: WorldAreaWeather,
    world_area_time: WorldAreaTime,
    base_version: BaseVersion,
    steam_id: u64,
    ps5_activity: PS5Activity,
    dlc: DLC,
    player_data_hash: PlayerGameDataHash,
    rest: Vec<u8>,
}

impl CharacterSlot {
    fn read_sp_effects<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Box<[SPEffect; 13]>> {
        // Vec -> boxed array, infallible here because we push exactly 13
        let mut v = Vec::with_capacity(13);
        for _ in 0..13 {
            v.push(SPEffect::read(cursor)?);
        }
        // have to map Vec error -> io::Error
        v.into_boxed_slice()
            .try_into()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "sp_effects len"))
    }

    fn read_event_flags<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Vec<u8>> {
        const LEN: usize = 1_833_375;
        let mut v = vec![0u8; LEN];
        cursor.read_exact(&mut v)?;
        Ok(v)
    }

    fn parse_gaitem_map<R: Read>(
        cursor: &mut CursorReader<R>,
        count: usize,
    ) -> io::Result<Vec<Gaitem>> {
        let mut gaitems: Vec<Gaitem> = Vec::with_capacity(count);

        for _ in 0..count {
            let gaitem_handle = cursor.read_u32_le()?;

            let item_id = cursor.read_u32_le()?;

            let mut item = Gaitem {
                gaitem_handle,
                item_id,
                ..Default::default()
            };

            if gaitem_handle != 0 {
                let handle_type = gaitem_handle & 0xF0000000;

                if gaitem_handle != 0 && handle_type != 0xC0000000 {
                    let unk_0x10 = cursor.read_u32_le()?;

                    item.unk_0x10 = Some(unk_0x10);

                    let unk_0x14 = cursor.read_u32_le()?;

                    item.unk_0x14 = Some(unk_0x14);

                    if handle_type == 0x80000000 {
                        let gem_gaitem_handle = cursor.read_u32_le()?;

                        item.gem_gaitem_handle = Some(gem_gaitem_handle);

                        let unk_0x1c = cursor.read_u8_le()?;

                        item.unk_0x1c = Some(unk_0x1c);
                    }
                }

                // println!("{:?}", item);
            }

            gaitems.push(item);
        }

        Ok(gaitems)
    }

    fn read_unk0x8<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<[u8; 8]> {
        let mut unk0x8 = [0u8; 8];

        cursor.read_exact(&mut unk0x8)?;

        Ok(unk0x8)
    }

    fn read_unk0x10<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<[u8; 16]> {
        let mut unk0x10 = [0u8; 16];

        cursor.read_exact(&mut unk0x10)?;

        Ok(unk0x10)
    }

    pub fn read_gestures<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Vec<u32>> {
        let mut gestures: Vec<u32> = Vec::with_capacity(GESTURES);

        for _ in 0..GESTURES {
            gestures.push(cursor.read_u32_le()?)
        }

        Ok(gestures)
    }

    pub fn read_equipped_gestures<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Vec<u32>> {
        let mut equipped_gestures: Vec<u32> = Vec::with_capacity(EQUIPPED_GESTURES);

        for _ in 0..EQUIPPED_GESTURES {
            equipped_gestures.push(cursor.read_u32_le()?)
        }

        Ok(equipped_gestures)
    }

    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        let slot_start = cursor.position();
        let version = cursor.read_u32_le()?;
        let gaitem_count = if version <= 81 {
            GAITEM_COUNT_OLD
        } else {
            GAITEM_COUNT_NEW
        };

        let mut slot = Self {
            version,
            map_id: cursor.read_u32_le()?,
            unk0x8: Self::read_unk0x8(cursor)?,
            unk0x10: Self::read_unk0x10(cursor)?,
            gaitem_map: Self::parse_gaitem_map(cursor, gaitem_count)?,
            player_game_data: PlayerGameData::read(cursor)?,
            sp_effects: Self::read_sp_effects(cursor)?,
            equipped_items_equip_index: EquipmentSlots::read(cursor)?,
            active_weapon_slots_and_arm: ActiveWeaponSlotsAndArmStyle::read(cursor)?,
            equipped_items_item_id: EquipmentSlots::read(cursor)?,
            equipped_items_gaitem_handle: EquipmentSlots::read(cursor)?,
            inventory_held: Inventory::read_inventory_held(cursor)?,
            equipped_spells: EquippedSpells::read(cursor)?,
            equipped_items: EquippedItems::read(cursor)?,
            equipped_gestures: Self::read_equipped_gestures(cursor)?,
            acquired_projectiles: AcquiredProjectiles::read(cursor)?,
            equipped_armaments_and_items: EquippedArmamentsAndItems::read(cursor)?,
            equipped_physics: EquippedPhysics::read(cursor)?,
            face_data: Self::read_face_data(cursor)?,
            inventory_storage_box: Inventory::read_inventory_storage(cursor)?,
            gestures: Self::read_gestures(cursor)?,
            unlocked_regions: Regions::read(cursor)?,
            horse: RideGameData::read(cursor)?,
            control_byte_maybe: cursor.read_u8_le()?,
            blood_stain: BloodStain::read(cursor)?,
            unk_gamedataman_0x120: cursor.read_u32_le()?,
            unk_gamedataman_0x88: cursor.read_u32_le()?,
            menu_profile_save_load: MenuSaveLoad::read(cursor)?,
            trophy_equip_data: TrophyEquipData::read(cursor)?,
            gaitem_game_data: GaitemGameData::read(cursor)?,
            tutorial_data: TutorialData::read(cursor)?,
            gameman_0x8c: cursor.read_u8_le()?,
            gameman_0x8d: cursor.read_u8_le()?,
            gameman_0x8e: cursor.read_u8_le()?,
            total_deaths_count: cursor.read_u32_le()?,
            character_type: cursor.read_i32_le()?,
            in_online_session_flag: cursor.read_u8_le()?,
            character_type_online: cursor.read_u32_le()?,
            last_rested_grace: cursor.read_u32_le()?,
            not_alone_flag: cursor.read_u8_le()?,
            in_game_countdown_timer: cursor.read_u32_le()?,
            unk_gamedataman_0x124: cursor.read_u32_le()?,
            event_flags: Self::read_event_flags(cursor)?, // 1_833_375
            event_flags_terminator: cursor.read_u8_le()?,
            field_area: FieldArea::read(cursor)?,
            world_area: WorldArea::read(cursor)?,
            world_geom_man: WorldGeomMan::read(cursor)?,
            world_geom_man2: WorldGeomMan::read(cursor)?,
            rend_man: RendMan::read(cursor)?,
            player_coordinates: PlayerCoordinates::read(cursor)?,
            game_man_0x5be: cursor.read_u8_le()?,
            game_man_0x5bf: cursor.read_u8_le()?,
            spawn_point_entity_id: cursor.read_u32_le()?,
            game_man_0xb64: cursor.read_u32_le()?,
            temp_spawn_point_entity_id: if version >= 65 {
                Some(cursor.read_u32_le()?)
            } else {
                None
            },
            game_man_0xcb3: if version >= 66 {
                Some(cursor.read_u8_le()?)
            } else {
                None
            },
            net_man: NetMan::read(cursor)?,
            world_area_weather: WorldAreaWeather::read(cursor)?,
            world_area_time: WorldAreaTime::read(cursor)?,
            base_version: BaseVersion::read(cursor)?,
            steam_id: cursor.read_u64_le()?,
            ps5_activity: PS5Activity::read(cursor)?,
            dlc: DLC::read(cursor)?,
            player_data_hash: PlayerGameDataHash::read(cursor)?,
            rest: Vec::new(),
        };

        let slot_end = slot_start + (PC_CHARACTER_SLOT_SIZE - CHECKSUM_SIZE) as u64;
        let position = cursor.position();

        if position > slot_end {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "character slot overran its end: position {}, slot end {}",
                    position, slot_end
                ),
            ));
        }

        slot.rest = vec![0u8; (slot_end - position) as usize];
        cursor.read_exact(&mut slot.rest)?;

        Ok(slot)
    }

    fn read_face_data<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<[u8; 303]> {
        let mut buffer = [0u8; 303];

        cursor.read_exact(&mut buffer)?;

        Ok(buffer)
    }
}

#[derive(Debug, Default)]
pub struct Inventory {
    common_item_count: u32,
    common_items: Vec<InventoryItem>,
    key_item_count: u32,
    key_items: Vec<InventoryItem>,
    equip_index_counter: u32,
    acquisition_index_counter: u32,
}

impl Inventory {
    pub fn read_inventory_held<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            common_item_count: cursor.read_u32_le()?,
            common_items: Self::read_common_items_held(cursor)?,
            key_item_count: cursor.read_u32_le()?,
            key_items: Self::read_key_items_held(cursor)?,
            equip_index_counter: cursor.read_u32_le()?,
            acquisition_index_counter: cursor.read_u32_le()?,
        })
    }

    pub fn read_inventory_storage<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            common_item_count: cursor.read_u32_le()?,
            common_items: Self::read_common_items_storage(cursor)?,
            key_item_count: cursor.read_u32_le()?,
            key_items: Self::read_key_items_storage(cursor)?,
            equip_index_counter: cursor.read_u32_le()?,
            acquisition_index_counter: cursor.read_u32_le()?,
        })
    }

    fn read_common_items_held<R: Read>(
        cursor: &mut CursorReader<R>,
    ) -> io::Result<Vec<InventoryItem>> {
        let mut common_items_held: Vec<InventoryItem> =
            Vec::with_capacity(INVENTORY_HELD_COMMON_SLOT_COUNT);

        for _ in 0..INVENTORY_HELD_COMMON_SLOT_COUNT {
            common_items_held.push(InventoryItem::read(cursor)?)
        }

        Ok(common_items_held)
    }

    fn read_common_items_storage<R: Read>(
        cursor: &mut CursorReader<R>,
    ) -> io::Result<Vec<InventoryItem>> {
        let mut common_items_storage: Vec<InventoryItem> =
            Vec::with_capacity(INVENTORY_STORAGE_COMMON_SLOT_COUNT);

        for _ in 0..INVENTORY_STORAGE_COMMON_SLOT_COUNT {
            common_items_storage.push(InventoryItem::read(cursor)?)
        }

        Ok(common_items_storage)
    }

    fn read_key_items_held<R: Read>(
        cursor: &mut CursorReader<R>,
    ) -> io::Result<Vec<InventoryItem>> {
        let mut key_items_held: Vec<InventoryItem> =
            Vec::with_capacity(INVENTORY_HELD_KEY_SLOT_COUNT);

        for _ in 0..INVENTORY_HELD_KEY_SLOT_COUNT {
            key_items_held.push(InventoryItem::read(cursor)?)
        }

        Ok(key_items_held)
    }

    fn read_key_items_storage<R: Read>(
        cursor: &mut CursorReader<R>,
    ) -> io::Result<Vec<InventoryItem>> {
        let mut key_items_storage: Vec<InventoryItem> =
            Vec::with_capacity(INVENTORY_STORAGE_KEY_SLOT_COUNT);

        for _ in 0..INVENTORY_STORAGE_KEY_SLOT_COUNT {
            key_items_storage.push(InventoryItem::read(cursor)?)
        }

        Ok(key_items_storage)
    }
}

#[derive(Debug, Default)]
pub struct InventoryItem {
    gaitem_handle: u32,
    quantity: u32,
    acquisition_index: u32,
}

impl InventoryItem {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            gaitem_handle: cursor.read_u32_le()?,
            quantity: cursor.read_u32_le()?,
            acquisition_index: cursor.read_u32_le()?,
        })
    }
}

#[derive(Debug)]
pub struct EquippedSpells {
    spell_slots: Vec<Spell>,
    active_index: u32,
}

impl EquippedSpells {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            spell_slots: Self::read_spell(cursor)?,
            active_index: cursor.read_u32_le()?,
        })
    }

    fn read_spell<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Vec<Spell>> {
        let mut spells: Vec<Spell> = Vec::with_capacity(EQUIPPED_SPELL_COUNT);

        for _ in 0..EQUIPPED_SPELL_COUNT {
            spells.push(Spell::read(cursor)?)
        }

        Ok(spells)
    }
}

#[derive(Debug)]
pub struct Spell {
    spell_id: u32,
    unk0x4: u32,
}

impl Spell {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            spell_id: cursor.read_u32_le()?,
            unk0x4: cursor.read_u32_le()?,
        })
    }
}

#[derive(Debug)]
pub struct EquippedItems {
    quick_items: Vec<EquippedItem>,
    active_quick_item_index: u32,
    pouch_items: Vec<EquippedItem>,
    unk0x84: u32,
    unk0x88: u32,
}

impl EquippedItems {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            quick_items: Self::read_quick_items(cursor)?,
            active_quick_item_index: cursor.read_u32_le()?,
            pouch_items: Self::read_pouch_items(cursor)?,
            unk0x84: cursor.read_u32_le()?,
            unk0x88: cursor.read_u32_le()?,
        })
    }

    pub fn read_quick_items<R: Read>(
        cursor: &mut CursorReader<R>,
    ) -> io::Result<Vec<EquippedItem>> {
        let mut quick_items: Vec<EquippedItem> = Vec::with_capacity(EQUIPPED_QUICK_ITEMS);

        for _ in 0..EQUIPPED_QUICK_ITEMS {
            quick_items.push(EquippedItem::read(cursor)?);
        }

        Ok(quick_items)
    }

    pub fn read_pouch_items<R: Read>(
        cursor: &mut CursorReader<R>,
    ) -> io::Result<Vec<EquippedItem>> {
        let mut pouch_items: Vec<EquippedItem> = Vec::with_capacity(EQUIPPED_POUCH_ITEMS);

        for _ in 0..EQUIPPED_POUCH_ITEMS {
            pouch_items.push(EquippedItem::read(cursor)?);
        }

        Ok(pouch_items)
    }
}

#[derive(Debug)]
pub struct EquippedItem {
    gaitem_handle: u32,
    equip_index: u32,
}

impl EquippedItem {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(EquippedItem {
            gaitem_handle: cursor.read_u32_le()?,
            equip_index: cursor.read_u32_le()?,
        })
    }
}

#[derive(Debug)]
pub struct AcquiredProjectiles {
    count: u32,
    projectiles: Vec<Projectile>,
}

impl AcquiredProjectiles {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        let count: u32 = cursor.read_u32_le()?;

        let mut projectiles: Vec<Projectile> = Vec::with_capacity(count as usize);

        for _ in 0..count {
            projectiles.push(Projectile::read(cursor)?);
        }

        Ok(Self { count, projectiles })
    }
}

#[derive(Debug)]
pub struct Projectile {
    id: u32,
    unk0x4: u32,
}

impl Projectile {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            id: cursor.read_u32_le()?,
            unk0x4: cursor.read_u32_le()?,
        })
    }
}

#[derive(Debug)]
pub struct EquippedArmamentsAndItems {
    left_hand_armament1: u32,
    right_hand_armament1: u32,
    left_hand_armament2: u32,
    right_hand_armament2: u32,
    left_hand_armament3: u32,
    right_hand_armament3: u32,
    arrows1: u32,
    bolts1: u32,
    arrows2: u32,
    bolts2: u32,
    arrows3: u32,
    bolts3: u32,
    head: u32,
    chest: u32,
    arms: u32,
    legs: u32,
    hair: u32,
    talisman1: u32,
    talisman2: u32,
    talisman3: u32,
    talisman4: u32,
    talisman5: u32,
    quickitem1: u32,
    quickitem2: u32,
    quickitem3: u32,
    quickitem4: u32,
    quickitem5: u32,
    quickitem6: u32,
    quickitem7: u32,
    quickitem8: u32,
    quickitem9: u32,
    quickitem10: u32,
    pouch1: u32,
    pouch2: u32,
    pouch3: u32,
    pouch4: u32,
    pouch5: u32,
    pouch6: u32,
    unk0x98: u32,
}

impl EquippedArmamentsAndItems {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            left_hand_armament1: cursor.read_u32_le()?,
            right_hand_armament1: cursor.read_u32_le()?,
            left_hand_armament2: cursor.read_u32_le()?,
            right_hand_armament2: cursor.read_u32_le()?,
            left_hand_armament3: cursor.read_u32_le()?,
            right_hand_armament3: cursor.read_u32_le()?,
            arrows1: cursor.read_u32_le()?,
            bolts1: cursor.read_u32_le()?,
            arrows2: cursor.read_u32_le()?,
            bolts2: cursor.read_u32_le()?,
            arrows3: cursor.read_u32_le()?,
            bolts3: cursor.read_u32_le()?,
            head: cursor.read_u32_le()?,
            chest: cursor.read_u32_le()?,
            arms: cursor.read_u32_le()?,
            legs: cursor.read_u32_le()?,
            hair: cursor.read_u32_le()?,
            talisman1: cursor.read_u32_le()?,
            talisman2: cursor.read_u32_le()?,
            talisman3: cursor.read_u32_le()?,
            talisman4: cursor.read_u32_le()?,
            talisman5: cursor.read_u32_le()?,
            quickitem1: cursor.read_u32_le()?,
            quickitem2: cursor.read_u32_le()?,
            quickitem3: cursor.read_u32_le()?,
            quickitem4: cursor.read_u32_le()?,
            quickitem5: cursor.read_u32_le()?,
            quickitem6: cursor.read_u32_le()?,
            quickitem7: cursor.read_u32_le()?,
            quickitem8: cursor.read_u32_le()?,
            quickitem9: cursor.read_u32_le()?,
            quickitem10: cursor.read_u32_le()?,
            pouch1: cursor.read_u32_le()?,
            pouch2: cursor.read_u32_le()?,
            pouch3: cursor.read_u32_le()?,
            pouch4: cursor.read_u32_le()?,
            pouch5: cursor.read_u32_le()?,
            pouch6: cursor.read_u32_le()?,
            unk0x98: cursor.read_u32_le()?,
        })
    }
}

#[derive(Debug)]
pub struct EquippedPhysics {
    slot1: u32,
    slot2: u32,
    unk0x8: u32,
}

impl EquippedPhysics {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            slot1: cursor.read_u32_le()?,
            slot2: cursor.read_u32_le()?,
            unk0x8: cursor.read_u32_le()?,
        })
    }
}

#[derive(Debug)]
pub struct Regions {
    count: u32,
    region_ids: Vec<u32>,
}

impl Regions {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        let count: u32 = cursor.read_u32_le()?;

        let mut region_ids: Vec<u32> = Vec::with_capacity(count as usize);

        for _ in 0..count {
            region_ids.push(cursor.read_u32_le()?);
        }

        Ok(Self { count, region_ids })
    }
}

#[derive(Debug)]
pub struct RideGameData {
    coordinates: Vec<f32>,
    map_id: u32,
    angle: Vec<f32>,
    hp: i32,
    state: u32,
}

impl RideGameData {
    fn read_coordinates<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Vec<f32>> {
        let mut coordinates: Vec<f32> = Vec::with_capacity(3);

        for _ in 0..3 {
            coordinates.push(cursor.read_f32_le()?);
        }

        Ok(coordinates)
    }

    fn read_angles<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Vec<f32>> {
        let mut angles: Vec<f32> = Vec::with_capacity(4);

        for _ in 0..4 {
            angles.push(cursor.read_f32_le()?);
        }

        Ok(angles)
    }

    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            coordinates: Self::read_coordinates(cursor)?,
            map_id: cursor.read_u32_le()?,
            angle: Self::read_angles(cursor)?,
            hp: cursor.read_i32_le()?,
            state: cursor.read_u32_le()?,
        })
    }
}

#[derive(Debug)]
pub struct BloodStain {
    coordinates: Vec<f32>,
    angle: Vec<f32>,
    unk0x1c: u32,
    unk0x20: u32,
    unk0x24: u32,
    unk0x28: u32,
    unk0x2c: u32,
    unk0x30: u32,
    runes: u32,
    map_id: u32,
    unk0x3c: u32,
    unk0x38: u32,
}

impl BloodStain {
    fn read_coordinates<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Vec<f32>> {
        let mut coordinates: Vec<f32> = Vec::with_capacity(3);

        for _ in 0..3 {
            coordinates.push(cursor.read_f32_le()?);
        }

        Ok(coordinates)
    }

    fn read_angles<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Vec<f32>> {
        let mut angles: Vec<f32> = Vec::with_capacity(4);

        for _ in 0..4 {
            angles.push(cursor.read_f32_le()?);
        }

        Ok(angles)
    }

    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            coordinates: Self::read_coordinates(cursor)?,
            angle: Self::read_angles(cursor)?,
            unk0x1c: cursor.read_u32_le()?,
            unk0x20: cursor.read_u32_le()?,
            unk0x24: cursor.read_u32_le()?,
            unk0x28: cursor.read_u32_le()?,
            unk0x2c: cursor.read_u32_le()?,
            unk0x30: cursor.read_u32_le()?,
            runes: cursor.read_u32_le()?,
            map_id: cursor.read_u32_le()?,
            unk0x3c: cursor.read_u32_le()?,
            unk0x38: cursor.read_u32_le()?,
        })
    }
}

#[derive(Debug)]
pub struct MenuSaveLoad {
    unk0x0: u16,
    unk0x2: u16,
    size: u32,
    data: Vec<u8>,
}

impl MenuSaveLoad {
    pub fn read_data<R: Read>(cursor: &mut CursorReader<R>, size: u32) -> io::Result<Vec<u8>> {
        let size = if size > 0x10000 { 0x1000 } else { size };
        let mut data = vec![0u8; size as usize];
        cursor.read_exact(&mut data)?;
        Ok(data)
    }

    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        let unk0x0 = cursor.read_u16_le()?;
        let unk0x2 = cursor.read_u16_le()?;
        let size = cursor.read_u32_le()?;

        Ok(Self {
            unk0x0,
            unk0x2,
            size,
            data: Self::read_data(cursor, size)?,
        })
    }
}

#[derive(Debug)]
pub struct TrophyEquipData {
    unk0x0: u32,
    legendary_weapon: u128,
    legendary_goods: u128,
    legendary_talismans: u128,
}

impl TrophyEquipData {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            unk0x0: cursor.read_u32_le()?,
            legendary_weapon: cursor.read_u128_le()?,
            legendary_goods: cursor.read_u128_le()?,
            legendary_talismans: cursor.read_u128_le()?,
        })
    }
}

#[derive(Debug)]
pub struct GaitemGameData {
    count: u64,
    entries: Box<[GaitemGameDataEntry; 14000]>,
}

impl GaitemGameData {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            count: cursor.read_u64_le()?,
            entries: Self::read_entries(cursor)?,
        })
    }

    pub fn read_entries<R: Read>(
        cursor: &mut CursorReader<R>,
    ) -> io::Result<Box<[GaitemGameDataEntry; 14000]>> {
        let mut entries: Vec<GaitemGameDataEntry> = Vec::with_capacity(14000);

        for _ in 0..14000 {
            entries.push(GaitemGameDataEntry::read(cursor)?);
        }

        entries
            .into_boxed_slice()
            .try_into()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "entries len"))
    }
}

#[derive(Debug)]
pub struct GaitemGameDataEntry {
    item_id: u32,
    acquired: u8,
    padding: [u8; 3],
}

impl GaitemGameDataEntry {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            item_id: cursor.read_u32_le()?,
            acquired: cursor.read_u8_le()?,
            padding: Self::read_padding(cursor)?,
        })
    }

    pub fn read_padding<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<[u8; 3]> {
        let mut padding = [0u8; 3];
        cursor.read_exact(&mut padding)?;
        Ok(padding)
    }
}

#[derive(Debug)]
pub struct TutorialData {
    unk0x0: u16,
    unk0x2: u16,
    size: u32,
    data: TutorialDataChunk,
}

impl TutorialData {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        let unk0x0 = cursor.read_u16_le()?;
        let unk0x2 = cursor.read_u16_le()?;
        let size = cursor.read_u32_le()?;
        let size = if size > 0x10000 { 0x400 } else { size };

        Ok(Self {
            unk0x0,
            unk0x2,
            size,
            data: TutorialDataChunk::read(cursor, size)?,
        })
    }
}

#[derive(Debug)]
pub struct TutorialDataChunk {
    count: u32,
    tutorial_ids: Vec<u32>,
}

impl TutorialDataChunk {
    pub fn read_tutorial_ids<R: Read>(
        cursor: &mut CursorReader<R>,
        size: u32,
    ) -> io::Result<Vec<u32>> {
        let id_count = (size.saturating_sub(4) / 4) as usize;
        let mut tutorial_ids: Vec<u32> = Vec::with_capacity(id_count);

        for _ in 0..id_count {
            tutorial_ids.push(cursor.read_u32_le()?);
        }

        Ok(tutorial_ids)
    }

    pub fn read<R: Read>(cursor: &mut CursorReader<R>, size: u32) -> io::Result<Self> {
        Ok(Self {
            count: cursor.read_u32_le()?,
            tutorial_ids: Self::read_tutorial_ids(cursor, size)?,
        })
    }
}

#[derive(Debug)]
pub struct FieldArea {
    size: u32,
    data: Vec<u8>,
}

impl FieldArea {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        let size = cursor.read_u32_le()?;
        let data = if size > 0 && size < 0x10000 {
            Self::read_data(cursor, size)?
        } else {
            Vec::new()
        };

        Ok(Self { size, data })
    }

    pub fn read_data<R: Read>(cursor: &mut CursorReader<R>, size: u32) -> io::Result<Vec<u8>> {
        let mut data = vec![0u8; size as usize];
        cursor.read_exact(&mut data)?;
        Ok(data)
    }
}

#[derive(Debug)]
pub struct WorldArea {
    size: u32,
    data: Vec<u8>,
}

impl WorldArea {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        let size = cursor.read_u32_le()?;
        let data = if size > 0 && size < 0x10000 {
            Self::read_data(cursor, size)?
        } else {
            Vec::new()
        };

        Ok(Self { size, data })
    }

    pub fn read_data<R: Read>(cursor: &mut CursorReader<R>, size: u32) -> io::Result<Vec<u8>> {
        let mut data = vec![0u8; size as usize];
        cursor.read_exact(&mut data)?;
        Ok(data)
    }
}

#[derive(Debug)]
pub struct WorldGeomMan {
    size: u32,
    data: Vec<u8>,
}

impl WorldGeomMan {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        let size = cursor.read_u32_le()?;
        let data = if size > 0 && size < 0x100000 {
            Self::read_data(cursor, size)?
        } else {
            Vec::new()
        };

        Ok(Self { size, data })
    }

    pub fn read_data<R: Read>(cursor: &mut CursorReader<R>, size: u32) -> io::Result<Vec<u8>> {
        let mut data = vec![0u8; size as usize];
        cursor.read_exact(&mut data)?;
        Ok(data)
    }
}

#[derive(Debug)]
pub struct RendMan {
    size: u32,
    data: Vec<u8>,
}

impl RendMan {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        let size = cursor.read_u32_le()?;
        let data = if size > 0 && size < 0x100000 {
            Self::read_data(cursor, size)?
        } else {
            Vec::new()
        };

        Ok(Self { size, data })
    }

    pub fn read_data<R: Read>(cursor: &mut CursorReader<R>, size: u32) -> io::Result<Vec<u8>> {
        let mut data = vec![0u8; size as usize];
        cursor.read_exact(&mut data)?;
        Ok(data)
    }
}

#[derive(Debug)]
pub struct PlayerCoordinates {
    coordinates: [f32; 3],
    map_id: u32,
    angle: [f32; 4],
    game_man_0xbf0: u8,
    unk_coordinates: [f32; 3],
    unk_angle: [f32; 4],
}

impl PlayerCoordinates {
    pub fn read_coordinates<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<[f32; 3]> {
        let mut coordinates = [0f32; 3];
        for i in 0..coordinates.len() {
            coordinates[i] = cursor.read_f32_le()?;
        }

        Ok(coordinates)
    }

    pub fn read_angle<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<[f32; 4]> {
        let mut angles = [0f32; 4];
        for i in 0..angles.len() {
            angles[i] = cursor.read_f32_le()?;
        }

        Ok(angles)
    }

    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            coordinates: Self::read_coordinates(cursor)?,
            map_id: cursor.read_u32_le()?,
            angle: Self::read_angle(cursor)?,
            game_man_0xbf0: cursor.read_u8_le()?,
            unk_coordinates: Self::read_coordinates(cursor)?,
            unk_angle: Self::read_angle(cursor)?,
        })
    }
}

#[derive(Debug)]
pub struct NetMan {
    unk0x0: u32,
    data: Box<[u8; 0x20_000]>,
}

impl NetMan {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            unk0x0: cursor.read_u32_le()?,
            data: Self::read_data(cursor)?,
        })
    }

    pub fn read_data<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Box<[u8; 0x20_000]>> {
        let mut data = vec![0u8; 0x20_000 as usize];

        cursor.read_exact(&mut data)?;

        data.into_boxed_slice()
            .try_into()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "netman len"))
    }
}

#[derive(Debug)]
pub struct WorldAreaWeather {
    area_id: u16,
    weather_type: u16,
    timer: u32,
    padding: u32,
}

impl WorldAreaWeather {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            area_id: cursor.read_u16_le()?,
            weather_type: cursor.read_u16_le()?,
            timer: cursor.read_u32_le()?,
            padding: cursor.read_u32_le()?,
        })
    }
}

#[derive(Debug)]
pub struct WorldAreaTime {
    hour: u32,
    minute: u32,
    second: u32,
}

impl WorldAreaTime {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            hour: cursor.read_u32_le()?,
            minute: cursor.read_u32_le()?,
            second: cursor.read_u32_le()?,
        })
    }
}

#[derive(Debug)]
pub struct BaseVersion {
    base_version_copy: u32,
    base_version: u32,
    is_latest_version: u32,
    unk0xc: u32,
}

impl BaseVersion {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            base_version_copy: cursor.read_u32_le()?,
            base_version: cursor.read_u32_le()?,
            is_latest_version: cursor.read_u32_le()?,
            unk0xc: cursor.read_u32_le()?,
        })
    }
}

#[derive(Debug)]
pub struct PS5Activity {
    data: [u8; 0x20],
}

impl PS5Activity {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        let mut data = [0u8; 0x20 as usize];

        cursor.read_exact(&mut data)?;

        Ok(Self { data })
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub struct DLC {
    preorder_the_ring: u8,
    shadow_of_the_erdtree: u8,
    preorder_ring_of_miquella: u8,
    tarnished_pack: u8,
    unused: [u8; 46],
}

impl DLC {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            preorder_the_ring: cursor.read_u8_le()?,
            shadow_of_the_erdtree: cursor.read_u8_le()?,
            preorder_ring_of_miquella: cursor.read_u8_le()?,
            tarnished_pack: cursor.read_u8_le()?,
            unused: Self::read_unused(cursor)?,
        })
    }

    pub fn read_unused<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<[u8; 46]> {
        let mut unused = [0u8; 46 as usize];

        cursor.read_exact(&mut unused)?;

        Ok(unused)
    }
}

#[derive(Debug)]
pub struct PlayerGameDataHash {
    leve: u32,
    stats: u32,
    archetype: u32,
    playergame_data_0xc0: u32,
    padding: u32,
    runes: u32,
    runes_memory: u32,
    equipped_weapons: u32,
    equipped_armors_and_talismans: u32,
    equipped_items: u32,
    equipped_spells: u32,
    rest: [u8; 0x54],
}

impl PlayerGameDataHash {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            leve: cursor.read_u32_le()?,
            stats: cursor.read_u32_le()?,
            archetype: cursor.read_u32_le()?,
            playergame_data_0xc0: cursor.read_u32_le()?,
            padding: cursor.read_u32_le()?,
            runes: cursor.read_u32_le()?,
            runes_memory: cursor.read_u32_le()?,
            equipped_weapons: cursor.read_u32_le()?,
            equipped_armors_and_talismans: cursor.read_u32_le()?,
            equipped_items: cursor.read_u32_le()?,
            equipped_spells: cursor.read_u32_le()?,
            rest: Self::read_rest(cursor)?,
        })
    }

    pub fn read_rest<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<[u8; 0x54]> {
        let mut rest = [0u8; 0x54 as usize];

        cursor.read_exact(&mut rest)?;

        Ok(rest)
    }
}

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Debug)]
pub struct USER_DATA_10 {
    MD5: u128,
    version: u32,
    steam_id: u64,
    settings: Settings,
    menu_system_save_load: MenuSystemSaveLoad,
    profile_summary: ProfileSummary,
    pc_option_data: Box<[u8; 0xB2]>,
    key_config_save_load: KeyConfigSaveLoad,
}

impl USER_DATA_10 {
    pub fn read_pc_option_data<R: Read>(
        cursor: &mut CursorReader<R>,
    ) -> io::Result<Box<[u8; 0xB2]>> {
        let mut data = vec![0u8; 0xB2 as usize];

        cursor.read_exact(&mut data)?;

        data.into_boxed_slice()
            .try_into()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "netman len"))
    }

    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        let start_pos = cursor.position();

        let MD5 = cursor.read_u128_le()?;
        let version = cursor.read_u32_le()?;
        let steam_id = cursor.read_u64_le()?;

        let settings = Settings::read(cursor)?;

        cursor.discard((start_pos + 16 + 4 + 8 + 0x140 - cursor.position()) as usize)?;

        let menu_system_save_load = MenuSystemSaveLoad::read(cursor)?;
        let profile_summary = ProfileSummary::read(cursor)?;

        cursor.discard(5)?;

        let pc_option_data = Self::read_pc_option_data(cursor)?;
        let key_config_save_load = KeyConfigSaveLoad::read(cursor)?;

        cursor.discard(8)?;

        let bytes_read = cursor.position() - start_pos;

        let remaining = 0x60010u64.saturating_sub(bytes_read);

        if remaining > 0 {
            cursor.discard(remaining as usize)?
        };

        Ok(Self {
            MD5,
            version,
            steam_id,
            settings,
            menu_system_save_load,
            profile_summary,
            pc_option_data,
            key_config_save_load,
        })
    }
}

#[derive(Debug)]
pub struct KeyConfigSaveLoad {
    size: u32,
    data: Vec<u8>,
}

impl KeyConfigSaveLoad {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        cursor.discard(4)?;
        let size = cursor.read_u32_le()?;
        let data = if size > 0 && size < 0x100000 {
            Self::read_data(cursor, size)?
        } else {
            Vec::new()
        };

        Ok(Self { size, data })
    }

    pub fn read_data<R: Read>(cursor: &mut CursorReader<R>, size: u32) -> io::Result<Vec<u8>> {
        let mut data = vec![0u8; size as usize];
        cursor.read_exact(&mut data)?;
        Ok(data)
    }
}

#[derive(Debug)]
pub struct MenuSystemSaveLoad {
    raw_data: Box<[u8; 0x1808]>,
    // parsed: CSMenuSystemSaveLoad,
}

impl MenuSystemSaveLoad {
    pub fn read_raw_data<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Box<[u8; 0x1808]>> {
        let mut presets: Vec<u8> = Vec::with_capacity(0x1808);

        for _ in 0..0x1808 {
            presets.push(cursor.read_u8_le()?);
        }

        presets
            .into_boxed_slice()
            .try_into()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "raw_data len"))
    }

    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            raw_data: Self::read_raw_data(cursor)?,
        })
    }
}

#[derive(Debug)]
pub struct CSMenuSystemSaveLoad {
    unk0x0: u16,
    unk0x2: u16,
    size: u32,
    presets: Box<[FacePreset; 15]>,
    padding: Box<[u8; 1576]>,
}

impl CSMenuSystemSaveLoad {
    pub fn read_presets<R: Read>(
        cursor: &mut CursorReader<R>,
    ) -> io::Result<Box<[FacePreset; 15]>> {
        let mut presets: Vec<FacePreset> = Vec::with_capacity(15);

        for _ in 0..15 {
            presets.push(FacePreset::read(cursor)?);
        }

        presets
            .into_boxed_slice()
            .try_into()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "presets len"))
    }

    pub fn read_padding<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Box<[u8; 1576]>> {
        let mut presets: Vec<u8> = Vec::with_capacity(1576);

        for _ in 0..1576 {
            presets.push(cursor.read_u8_le()?);
        }

        presets
            .into_boxed_slice()
            .try_into()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "presets len"))
    }

    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            unk0x0: cursor.read_u16_le()?,
            unk0x2: cursor.read_u16_le()?,
            size: cursor.read_u32_le()?,
            presets: Self::read_presets(cursor)?,
            padding: Self::read_padding(cursor)?,
        })
    }
}

#[derive(Debug)]
pub struct FacePreset {
    unk0x00: [u8; 20],
    face_data_marker: i32,
    magic: u32,
    alignment: u8,
    size: u8,
    face_model: u8,
    hair_model: u8,
    unk0x14: u8,
    eyebrow_model: u8,
    beard_model: u8,
    eyepatch_model: u8,
    unk0x24: u8,
    unk0x28: u8,
    apparent_age: u8,
    facial_aesthetic: u8,
    form_emphasis: u8,
    unk0x2f: u8,
    brow_ridge_height: u8,
    inner_brow_ridge: u8,
    outer_brow_ridge: u8,
    cheekbone_height: u8,
    cheekbone_depth: u8,
    cheekbone_width: u8,
    cheekbone_protrusion: u8,
    cheeks: u8,
    chin_tip_position: u8,
    chin_length: u8,
    chin_protrusion: u8,
    chin_depth: u8,
    chin_size: u8,
    chin_height: u8,
    chin_width: u8,
    eye_position: u8,
    eye_size: u8,
    eye_slant: u8,
    eye_spacing: u8,
    nose_size: u8,
    nose_forehead_ratio: u8,
    unk0x45: u8,
    face_protrusion: u8,
    vertical_face_ratio: u8,
    facial_feature_slant: u8,
    horizontal_face_ratio: u8,
    unk0x4a: u8,
    forehead_depth: u8,
    forehead_protrusion: u8,
    unk0x4d: u8,
    jaw_protrusion: u8,
    jaw_width: u8,
    lower_jaw: u8,
    jaw_contour: u8,
    lip_shape: u8,
    lip_size: u8,
    lip_fullness: u8,
    mouth_expression: u8,
    lip_protrusion: u8,
    lip_thickness: u8,
    mouth_protrusion: u8,
    mouth_slant: u8,
    occlusion: u8,
    mouth_position: u8,
    mouth_width: u8,
    mouth_chin_distance: u8,
    nose_ridge_depth: u8,
    nose_ridge_length: u8,
    nose_position: u8,
    nose_tip_height: u8,
    nostril_slant: u8,
    nostril_size: u8,
    nostril_width: u8,
    nose_protrusion: u8,
    nose_bridge_height: u8,
    bridge_protrusion1: u8,
    bridge_protrusion2: u8,
    nose_bridge_width: u8,
    nose_height: u8,
    nose_slant: u8,
    unk0x6c: Box<[u8; 512]>,
    head_size: u8,
    chest_size: u8,
    abdomen_size: u8,
    arms_size: u8,
    legs_size: u8,
    unk0xb1: u16,
    skin_color_r: u8,
    skin_color_g: u8,
    skin_color_b: u8,
    skin_luster: u8,
    pores: u8,
    stubble: u8,
    dark_circles: u8,
    dark_circle_color_r: u8,
    dark_circle_color_g: u8,
    dark_circle_color_b: u8,
    cheeks_color_intensity: u8,
    cheek_color_r: u8,
    cheek_color_g: u8,
    cheek_color_b: u8,
    eye_liner: u8,
    eye_liner_color_r: u8,
    eye_liner_color_g: u8,
    eye_liner_color_b: u8,
    eye_shadow_lower: u8,
    eye_shadow_lower_color_r: u8,
    eye_shadow_lower_color_g: u8,
    eye_shadow_lower_color_b: u8,
    eye_shadow_upper: u8,
    eye_shadow_upper_color_r: u8,
    eye_shadow_upper_color_g: u8,
    eye_shadow_upper_color_b: u8,
    lip_stick: u8,
    lip_stick_color_r: u8,
    lip_stick_color_g: u8,
    lip_stick_color_b: u8,
    tattoo_mark_position_horizontal: u8,
    tattoo_mark_position_vertical: u8,
    tattoo_mark_angle: u8,
    tattoo_mark_expansion: u8,
    tattoo_mark_color_r: u8,
    tattoo_mark_color_g: u8,
    tattoo_mark_color_b: u8,
    unk0xd8: u8,
    tattoo_mark_flip: u8,
    body_hair: u8,
    body_hair_color_r: u8,
    body_hair_color_g: u8,
    body_hair_color_b: u8,
    right_iris_color_r: u8,
    right_iris_color_g: u8,
    right_iris_color_b: u8,
    right_iris_size: u8,
    right_eye_clouding: u8,
    right_eye_clouding_color_r: u8,
    right_eye_clouding_color_g: u8,
    right_eye_clouding_color_b: u8,
    right_eye_white_color_r: u8,
    right_eye_white_color_g: u8,
    right_eye_white_color_b: u8,
    right_eye_position: u8,
    left_iris_color_r: u8,
    left_iris_color_g: u8,
    left_iris_color_b: u8,
    left_iris_size: u8,
    left_eye_clouding: u8,
    left_eye_clouding_color_r: u8,
    left_eye_clouding_color_g: u8,
    left_eye_clouding_color_b: u8,
    left_eye_white_color_r: u8,
    left_eye_white_color_g: u8,
    left_eye_white_color_b: u8,
    left_eye_position: u8,
    hair_color_r: u8,
    hair_color_g: u8,
    hair_color_b: u8,
    luster: u8,
    hair_root_darkness: u8,
    white_hairs: u8,
    beard_color_r: u8,
    beard_color_g: u8,
    beard_color_b: u8,
    beard_luster: u8,
    beard_root_darkness: u8,
    beard_white_hairs: u8,
    brow_color_r: u8,
    brow_color_g: u8,
    brow_color_b: u8,
    brow_luster: u8,
    brow_root_darkness: u8,
    brow_white_hairs: u8,
    eye_lash_color_r: u8,
    eye_lash_color_g: u8,
    eye_lash_color_b: u8,
    eye_patch_color_r: u8,
    eye_patch_color_g: u8,
    eye_patch_color_b: u8,
    pad: [u8; 10],
}

impl FacePreset {
    pub fn read_pad<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<[u8; 10]> {
        let mut pad = [0u8; 10];

        cursor.read_exact(&mut pad)?;

        Ok(pad)
    }

    pub fn read_unk0x6c<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Box<[u8; 512]>> {
        let mut unk0x6c: Vec<u8> = vec![0u8; 512];

        cursor.read_exact(&mut unk0x6c)?;

        unk0x6c
            .into_boxed_slice()
            .try_into()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "FacePreset: unk0x6c len"))
    }

    pub fn read_unk0x00<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<[u8; 20]> {
        let mut unk0x00 = [0u8; 20];

        cursor.read_exact(&mut unk0x00)?;

        Ok(unk0x00)
    }

    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            unk0x00: Self::read_unk0x00(cursor)?,
            face_data_marker: cursor.read_i32_le()?,
            magic: cursor.read_u32_le()?,
            alignment: cursor.read_u8_le()?,
            size: cursor.read_u8_le()?,
            face_model: cursor.read_u8_le()?,
            hair_model: cursor.read_u8_le()?,
            unk0x14: cursor.read_u8_le()?,
            eyebrow_model: cursor.read_u8_le()?,
            beard_model: cursor.read_u8_le()?,
            eyepatch_model: cursor.read_u8_le()?,
            unk0x24: cursor.read_u8_le()?,
            unk0x28: cursor.read_u8_le()?,
            apparent_age: cursor.read_u8_le()?,
            facial_aesthetic: cursor.read_u8_le()?,
            form_emphasis: cursor.read_u8_le()?,
            unk0x2f: cursor.read_u8_le()?,
            brow_ridge_height: cursor.read_u8_le()?,
            inner_brow_ridge: cursor.read_u8_le()?,
            outer_brow_ridge: cursor.read_u8_le()?,
            cheekbone_height: cursor.read_u8_le()?,
            cheekbone_depth: cursor.read_u8_le()?,
            cheekbone_width: cursor.read_u8_le()?,
            cheekbone_protrusion: cursor.read_u8_le()?,
            cheeks: cursor.read_u8_le()?,
            chin_tip_position: cursor.read_u8_le()?,
            chin_length: cursor.read_u8_le()?,
            chin_protrusion: cursor.read_u8_le()?,
            chin_depth: cursor.read_u8_le()?,
            chin_size: cursor.read_u8_le()?,
            chin_height: cursor.read_u8_le()?,
            chin_width: cursor.read_u8_le()?,
            eye_position: cursor.read_u8_le()?,
            eye_size: cursor.read_u8_le()?,
            eye_slant: cursor.read_u8_le()?,
            eye_spacing: cursor.read_u8_le()?,
            nose_size: cursor.read_u8_le()?,
            nose_forehead_ratio: cursor.read_u8_le()?,
            unk0x45: cursor.read_u8_le()?,
            face_protrusion: cursor.read_u8_le()?,
            vertical_face_ratio: cursor.read_u8_le()?,
            facial_feature_slant: cursor.read_u8_le()?,
            horizontal_face_ratio: cursor.read_u8_le()?,
            unk0x4a: cursor.read_u8_le()?,
            forehead_depth: cursor.read_u8_le()?,
            forehead_protrusion: cursor.read_u8_le()?,
            unk0x4d: cursor.read_u8_le()?,
            jaw_protrusion: cursor.read_u8_le()?,
            jaw_width: cursor.read_u8_le()?,
            lower_jaw: cursor.read_u8_le()?,
            jaw_contour: cursor.read_u8_le()?,
            lip_shape: cursor.read_u8_le()?,
            lip_size: cursor.read_u8_le()?,
            lip_fullness: cursor.read_u8_le()?,
            mouth_expression: cursor.read_u8_le()?,
            lip_protrusion: cursor.read_u8_le()?,
            lip_thickness: cursor.read_u8_le()?,
            mouth_protrusion: cursor.read_u8_le()?,
            mouth_slant: cursor.read_u8_le()?,
            occlusion: cursor.read_u8_le()?,
            mouth_position: cursor.read_u8_le()?,
            mouth_width: cursor.read_u8_le()?,
            mouth_chin_distance: cursor.read_u8_le()?,
            nose_ridge_depth: cursor.read_u8_le()?,
            nose_ridge_length: cursor.read_u8_le()?,
            nose_position: cursor.read_u8_le()?,
            nose_tip_height: cursor.read_u8_le()?,
            nostril_slant: cursor.read_u8_le()?,
            nostril_size: cursor.read_u8_le()?,
            nostril_width: cursor.read_u8_le()?,
            nose_protrusion: cursor.read_u8_le()?,
            nose_bridge_height: cursor.read_u8_le()?,
            bridge_protrusion1: cursor.read_u8_le()?,
            bridge_protrusion2: cursor.read_u8_le()?,
            nose_bridge_width: cursor.read_u8_le()?,
            nose_height: cursor.read_u8_le()?,
            nose_slant: cursor.read_u8_le()?,
            unk0x6c: Self::read_unk0x6c(cursor)?,
            head_size: cursor.read_u8_le()?,
            chest_size: cursor.read_u8_le()?,
            abdomen_size: cursor.read_u8_le()?,
            arms_size: cursor.read_u8_le()?,
            legs_size: cursor.read_u8_le()?,
            unk0xb1: cursor.read_u16_le()?,
            skin_color_r: cursor.read_u8_le()?,
            skin_color_g: cursor.read_u8_le()?,
            skin_color_b: cursor.read_u8_le()?,
            skin_luster: cursor.read_u8_le()?,
            pores: cursor.read_u8_le()?,
            stubble: cursor.read_u8_le()?,
            dark_circles: cursor.read_u8_le()?,
            dark_circle_color_r: cursor.read_u8_le()?,
            dark_circle_color_g: cursor.read_u8_le()?,
            dark_circle_color_b: cursor.read_u8_le()?,
            cheeks_color_intensity: cursor.read_u8_le()?,
            cheek_color_r: cursor.read_u8_le()?,
            cheek_color_g: cursor.read_u8_le()?,
            cheek_color_b: cursor.read_u8_le()?,
            eye_liner: cursor.read_u8_le()?,
            eye_liner_color_r: cursor.read_u8_le()?,
            eye_liner_color_g: cursor.read_u8_le()?,
            eye_liner_color_b: cursor.read_u8_le()?,
            eye_shadow_lower: cursor.read_u8_le()?,
            eye_shadow_lower_color_r: cursor.read_u8_le()?,
            eye_shadow_lower_color_g: cursor.read_u8_le()?,
            eye_shadow_lower_color_b: cursor.read_u8_le()?,
            eye_shadow_upper: cursor.read_u8_le()?,
            eye_shadow_upper_color_r: cursor.read_u8_le()?,
            eye_shadow_upper_color_g: cursor.read_u8_le()?,
            eye_shadow_upper_color_b: cursor.read_u8_le()?,
            lip_stick: cursor.read_u8_le()?,
            lip_stick_color_r: cursor.read_u8_le()?,
            lip_stick_color_g: cursor.read_u8_le()?,
            lip_stick_color_b: cursor.read_u8_le()?,
            tattoo_mark_position_horizontal: cursor.read_u8_le()?,
            tattoo_mark_position_vertical: cursor.read_u8_le()?,
            tattoo_mark_angle: cursor.read_u8_le()?,
            tattoo_mark_expansion: cursor.read_u8_le()?,
            tattoo_mark_color_r: cursor.read_u8_le()?,
            tattoo_mark_color_g: cursor.read_u8_le()?,
            tattoo_mark_color_b: cursor.read_u8_le()?,
            unk0xd8: cursor.read_u8_le()?,
            tattoo_mark_flip: cursor.read_u8_le()?,
            body_hair: cursor.read_u8_le()?,
            body_hair_color_r: cursor.read_u8_le()?,
            body_hair_color_g: cursor.read_u8_le()?,
            body_hair_color_b: cursor.read_u8_le()?,
            right_iris_color_r: cursor.read_u8_le()?,
            right_iris_color_g: cursor.read_u8_le()?,
            right_iris_color_b: cursor.read_u8_le()?,
            right_iris_size: cursor.read_u8_le()?,
            right_eye_clouding: cursor.read_u8_le()?,
            right_eye_clouding_color_r: cursor.read_u8_le()?,
            right_eye_clouding_color_g: cursor.read_u8_le()?,
            right_eye_clouding_color_b: cursor.read_u8_le()?,
            right_eye_white_color_r: cursor.read_u8_le()?,
            right_eye_white_color_g: cursor.read_u8_le()?,
            right_eye_white_color_b: cursor.read_u8_le()?,
            right_eye_position: cursor.read_u8_le()?,
            left_iris_color_r: cursor.read_u8_le()?,
            left_iris_color_g: cursor.read_u8_le()?,
            left_iris_color_b: cursor.read_u8_le()?,
            left_iris_size: cursor.read_u8_le()?,
            left_eye_clouding: cursor.read_u8_le()?,
            left_eye_clouding_color_r: cursor.read_u8_le()?,
            left_eye_clouding_color_g: cursor.read_u8_le()?,
            left_eye_clouding_color_b: cursor.read_u8_le()?,
            left_eye_white_color_r: cursor.read_u8_le()?,
            left_eye_white_color_g: cursor.read_u8_le()?,
            left_eye_white_color_b: cursor.read_u8_le()?,
            left_eye_position: cursor.read_u8_le()?,
            hair_color_r: cursor.read_u8_le()?,
            hair_color_g: cursor.read_u8_le()?,
            hair_color_b: cursor.read_u8_le()?,
            luster: cursor.read_u8_le()?,
            hair_root_darkness: cursor.read_u8_le()?,
            white_hairs: cursor.read_u8_le()?,
            beard_color_r: cursor.read_u8_le()?,
            beard_color_g: cursor.read_u8_le()?,
            beard_color_b: cursor.read_u8_le()?,
            beard_luster: cursor.read_u8_le()?,
            beard_root_darkness: cursor.read_u8_le()?,
            beard_white_hairs: cursor.read_u8_le()?,
            brow_color_r: cursor.read_u8_le()?,
            brow_color_g: cursor.read_u8_le()?,
            brow_color_b: cursor.read_u8_le()?,
            brow_luster: cursor.read_u8_le()?,
            brow_root_darkness: cursor.read_u8_le()?,
            brow_white_hairs: cursor.read_u8_le()?,
            eye_lash_color_r: cursor.read_u8_le()?,
            eye_lash_color_g: cursor.read_u8_le()?,
            eye_lash_color_b: cursor.read_u8_le()?,
            eye_patch_color_r: cursor.read_u8_le()?,
            eye_patch_color_g: cursor.read_u8_le()?,
            eye_patch_color_b: cursor.read_u8_le()?,
            pad: Self::read_pad(cursor)?,
        })
    }
}

#[derive(Debug)]
pub struct Settings {
    camera_speed: u8,
    controller_vibration: u8,
    brightness: u8,
    sound_type: u8,
    music_volume: u8,
    sound_effects_volume: u8,
    voice_volume: u8,
    display_blood: u8,
    subtitles: u8,
    hud: u8,
    camera_x_axis: u8,
    camera_y_axis: u8,
    toggle_auto_lockon: u8,
    camera_auto_wall_recovery: u8,
    unk0xe: u8,
    unk0xf: u8,
    reset_camera_y_axis: u8,
    cinematic_effects: u8,
    unk0x12: u8,
    cross_region_play: u8,
    voice_chat: u8,
    display_player_names: u8,
    manual_attack_aim: u8,
    autotarget: u8,
    launchsettings: u8,
    send_summon_sign: u8,
    unk0x1a: u8,
    hdr: u8,
    hdr_adjust_brightness: u8,
    hdr_maximum_brightness: u8,
    hdr_adjust_saturation: u8,
    unk0x1f: u8,
    master_volume: u8,
    is_raytracing_on: u8,
    mark_new_items: u8,
    show_recent_tabs: u8,
    unk0x24: u64,
    unk0x2c: u16,
    show_tutorials: u8,
    camera_auto_rotation: u8,
}

impl Settings {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            camera_speed: cursor.read_u8_le()?,
            controller_vibration: cursor.read_u8_le()?,
            brightness: cursor.read_u8_le()?,
            sound_type: cursor.read_u8_le()?,
            music_volume: cursor.read_u8_le()?,
            sound_effects_volume: cursor.read_u8_le()?,
            voice_volume: cursor.read_u8_le()?,
            display_blood: cursor.read_u8_le()?,
            subtitles: cursor.read_u8_le()?,
            hud: cursor.read_u8_le()?,
            camera_x_axis: cursor.read_u8_le()?,
            camera_y_axis: cursor.read_u8_le()?,
            toggle_auto_lockon: cursor.read_u8_le()?,
            camera_auto_wall_recovery: cursor.read_u8_le()?,
            unk0xe: cursor.read_u8_le()?,
            unk0xf: cursor.read_u8_le()?,
            reset_camera_y_axis: cursor.read_u8_le()?,
            cinematic_effects: cursor.read_u8_le()?,
            unk0x12: cursor.read_u8_le()?,
            cross_region_play: cursor.read_u8_le()?,
            voice_chat: cursor.read_u8_le()?,
            display_player_names: cursor.read_u8_le()?,
            manual_attack_aim: cursor.read_u8_le()?,
            autotarget: cursor.read_u8_le()?,
            launchsettings: cursor.read_u8_le()?,
            send_summon_sign: cursor.read_u8_le()?,
            unk0x1a: cursor.read_u8_le()?,
            hdr: cursor.read_u8_le()?,
            hdr_adjust_brightness: cursor.read_u8_le()?,
            hdr_maximum_brightness: cursor.read_u8_le()?,
            hdr_adjust_saturation: cursor.read_u8_le()?,
            unk0x1f: cursor.read_u8_le()?,
            master_volume: cursor.read_u8_le()?,
            is_raytracing_on: cursor.read_u8_le()?,
            mark_new_items: cursor.read_u8_le()?,
            show_recent_tabs: cursor.read_u8_le()?,
            unk0x24: cursor.read_u64_le()?,
            unk0x2c: cursor.read_u16_le()?,
            show_tutorials: cursor.read_u8_le()?,
            camera_auto_rotation: cursor.read_u8_le()?,
        })
    }
}

#[derive(Debug)]
pub struct ProfileEquipment {
    raw_data: [u8; 0xE8],
}

impl ProfileEquipment {
    pub fn read_raw_data<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<[u8; 0xE8]> {
        let mut raw_data = [0u8; 0xE8];

        cursor.read_exact(&mut raw_data)?;

        Ok(raw_data)
    }

    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            raw_data: Self::read_raw_data(cursor)?,
        })
    }
}

#[derive(Debug)]
pub struct Profile {
    character_name: String, // (UTF-16LE, 16 chars max)
    terminator: u16,
    level: u32,
    seconds_played: u32,
    runes_memory: u32,
    map_id: [u8; 4],
    place_name_id: u32,
    face_data: [u8; 0x124],
    equipment: ProfileEquipment,
    body_type: u8,
    archetype: u8,
    starting_gift: u8,
    unk_padding: [u8; 7],
}

impl Profile {
    pub fn read_map_id<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<[u8; 4]> {
        let mut map_id = [0u8; 4];

        cursor.read_exact(&mut map_id)?;

        Ok(map_id)
    }

    pub fn read_face_data<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<[u8; 0x124]> {
        let mut face_data = [0u8; 0x124];

        cursor.read_exact(&mut face_data)?;

        Ok(face_data)
    }

    pub fn read_unk_padding<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<[u8; 7]> {
        let mut unk_padding = [0u8; 7];

        cursor.read_exact(&mut unk_padding)?;

        Ok(unk_padding)
    }

    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            character_name: cursor.read_utf16_le_to_string(16)?,
            terminator: cursor.read_u16_le()?,
            level: cursor.read_u32_le()?,
            seconds_played: cursor.read_u32_le()?,
            runes_memory: cursor.read_u32_le()?,
            map_id: Self::read_map_id(cursor)?,
            place_name_id: cursor.read_u32_le()?,
            face_data: Self::read_face_data(cursor)?,
            equipment: ProfileEquipment::read(cursor)?,
            body_type: cursor.read_u8_le()?,
            archetype: cursor.read_u8_le()?,
            starting_gift: cursor.read_u8_le()?,
            unk_padding: Self::read_unk_padding(cursor)?,
        })
    }
}

#[derive(Debug)]
pub struct ProfileSummary {
    active_profiles: [u8; 10],
    profiles: Box<[Profile; 10]>,
}

impl ProfileSummary {
    pub fn read_active_profiles<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<[u8; 10]> {
        let mut active_profiles = [0u8; 10];

        cursor.read_exact(&mut active_profiles)?;

        Ok(active_profiles)
    }

    pub fn read_profiles<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Box<[Profile; 10]>> {
        let mut profiles: Vec<Profile> = Vec::with_capacity(10);

        for _ in 0..10 {
            profiles.push(Profile::read(cursor)?);
        }

        profiles
            .into_boxed_slice()
            .try_into()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "profiles len"))
    }

    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            active_profiles: Self::read_active_profiles(cursor)?,
            profiles: Self::read_profiles(cursor)?,
        })
    }
}

#[derive(Debug)]
pub struct USER_DATA_11 {
    data: Box<[u8; 0x240010]>,
}

impl USER_DATA_11 {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        cursor.discard(16)?;

        let mut data_buf = vec![0u8; 0x240_010 as usize];

        cursor.read_exact(&mut data_buf)?;

        Ok(Self {
            data: data_buf
                .into_boxed_slice()
                .try_into()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "USER_DATA_11 len"))?,
        })
    }
}
