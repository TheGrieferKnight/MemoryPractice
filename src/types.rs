#![allow(dead_code)]

use std::io::{self, Read};

use crate::{
    constants::*,
    cursor::{self, CursorReader},
};

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
    gaitem_map: Box<[Gaitem; 5120]>,
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
    temp_spawn_point_entity_id: u32,
    game_man_0xcb3: u8,
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

    fn parse_gaitem_map<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Box<[Gaitem; 5120]>> {
        let mut gaitems: Vec<Gaitem> = Vec::with_capacity(5120);

        for _ in 0..GAITEM_COUNT_NEW {
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

        gaitems
            .into_boxed_slice()
            .try_into()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "sp_effects len"))
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
        Ok(Self {
            version: cursor.read_u32_le()?,
            map_id: cursor.read_u32_le()?,
            unk0x8: Self::read_unk0x8(cursor)?,
            unk0x10: Self::read_unk0x10(cursor)?,
            gaitem_map: Self::parse_gaitem_map(cursor)?,
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
            temp_spawn_point_entity_id: cursor.read_u32_le()?,
            game_man_0xcb3: cursor.read_u8_le()?,
            net_man: NetMan::read(cursor)?,
            world_area_weather: WorldAreaWeather::read(cursor)?,
            world_area_time: WorldAreaTime::read(cursor)?,
            base_version: BaseVersion::read(cursor)?,
            steam_id: cursor.read_u64_le()?,
            ps5_activity: PS5Activity::read(cursor)?,
            dlc: DLC::read(cursor)?,
            player_data_hash: PlayerGameDataHash::read(cursor)?,
            rest: Vec::new(),
        })
    }

    fn read_face_data<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<[u8; 303]> {
        let mut buffer = [0u8; 303];

        cursor.read_exact(&mut buffer);

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
        let common_items_held = Self::read_common_items_held(cursor)?;

        let key_items_held = Self::read_key_items_held(cursor)?;

        Ok(Self {
            common_item_count: common_items_held.len() as u32,
            common_items: common_items_held,
            key_item_count: key_items_held.len() as u32,
            key_items: key_items_held,
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
            pouch_items: Self::read_quick_items(cursor)?,
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
    unk0x0: u16,
    legendary_weapon: u128,
    legendary_goods: u128,
    legendary_talismans: u128,
}

impl TrophyEquipData {
    pub fn read<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {
            unk0x0: cursor.read_u16_le()?,
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
        let mut tutorial_ids: Vec<u32> = Vec::with_capacity(size as usize);

        for _ in 0..size {
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

        Ok(Self {
            size,
            data: Self::read_data(cursor, size)?,
        })
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

        Ok(Self {
            size,
            data: Self::read_data(cursor, size)?,
        })
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

        Ok(Self {
            size,
            data: Self::read_data(cursor, size)?,
        })
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

        Ok(Self {
            size,
            data: Self::read_data(cursor, size)?,
        })
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
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "entries len"))
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
    pub fn read<R: Read>(_cursor: &mut CursorReader<R>) -> io::Result<Self> {
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
pub struct BaseVersion {}

impl BaseVersion {
    pub fn read<R: Read>(_cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {})
    }
}

#[derive(Debug)]
pub struct PS5Activity {}

impl PS5Activity {
    pub fn read<R: Read>(_cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {})
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub struct DLC {}

impl DLC {
    pub fn read<R: Read>(_cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {})
    }
}

#[derive(Debug)]
pub struct PlayerGameDataHash {}

impl PlayerGameDataHash {
    pub fn read<R: Read>(_cursor: &mut CursorReader<R>) -> io::Result<Self> {
        Ok(Self {})
    }
}
