use crate::types::Gaitem;

use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

mod types;

const CHARACTER_SLOT_OFFSETS: [u64;1] = [0x300u64];

// ,0x280310u64,0x500320u64,0x780330u64,0xA00340u64,0xC80350u64,0xF00360u64,0x1180370u64,0x1400380u64,0x1680390u64

fn main() -> io::Result<()> {
    let mut file = File::open("./ER0000.cnv")?;

    file.seek(SeekFrom::Start(0))?;

    let mut magic_bytes = [0u8; 4];

    file.read_exact(&mut magic_bytes)?;

    magic_bytes.iter().for_each(|magic_byte| println!("Magic Byte: {:?}", magic_byte));

    file.seek(SeekFrom::Start(4))?;

    let mut header = [0u8; 764];

    file.read_exact(&mut header)?;

    println!("Header: {:?}", header);

    for cso in CHARACTER_SLOT_OFFSETS {
        file.seek(SeekFrom::Start(cso))?;

        let mut data = vec![0u8; 0x280010];

        file.read_exact(&mut data)?;

        let checksum = &data[..16];

        println!("Checksum: {:02x?}", checksum);

        let version_bytes = &data[16..20];

        let version = u32::from_le_bytes(
            version_bytes.try_into().unwrap()
        );

        println!("Version: {version}");

        let is_empty = version.eq(&0u32);

        println!("Character Slot Empty? {:?}", is_empty);

        if is_empty {
            break;
        }

        let map_id_bytes = &data[20..24];

        let map_id =  u32::from_le_bytes(
            map_id_bytes.try_into().unwrap()
        );

        println!("Map ID: {map_id}");

        // gaitem_map


        let gaitem_map_bytes = &data[48..48+5120*8];

        let mut item_offset = 0;

        let mut item_count = 0;

        while item_count < 5120*8-8 {
            let mut item_offset_to_add = 8;

            let gaitem_handle_bytes = &gaitem_map_bytes[item_offset..item_offset+4];

            let gaitem_handle =  u32::from_le_bytes(
                gaitem_handle_bytes.try_into().unwrap()
            );

            if gaitem_handle != 0 {
                let item_id_bytes = &gaitem_map_bytes[item_offset+4..item_offset+8];

                let item_id =  u32::from_le_bytes(
                    item_id_bytes.try_into().unwrap()
                );

                let mut item = Gaitem {
                    gaitem_handle,
                    item_id,
                    ..Default::default()
                };

                let handle_type = gaitem_handle & 0xF0000000;

                if gaitem_handle != 0 && handle_type != 0xC0000000 {
                    let unk_0x10 = &gaitem_map_bytes[item_offset+8..item_offset+12];

                    item.unk_0x10 =  Some(i32::from_le_bytes(
                        unk_0x10.try_into().unwrap()
                    ));

                    let unk_0x14 = &gaitem_map_bytes[item_offset+12..item_offset+16];

                    item.unk_0x14 =  Some(i32::from_le_bytes(
                        unk_0x14.try_into().unwrap()
                    ));

                    item_offset_to_add = 16;

                    if handle_type == 0x80000000 {

                        let gem_gaitem_handle = &gaitem_map_bytes[item_offset+16..item_offset+20];

                        item.gem_gaitem_handle =  Some(i32::from_le_bytes(
                            gem_gaitem_handle.try_into().unwrap()
                        ));

                        let unk_0x1c = &gaitem_map_bytes[item_offset+20..item_offset+21];

                        item.unk_0x1c =  Some(u8::from_le_bytes(
                            unk_0x1c.try_into().unwrap()
                        ));

                        item_offset_to_add = 21;

                    }

                }

                item_count += 1;

                println!("{:?}", item);
            }

            item_offset += item_offset_to_add;

        }

        println!("{:?}", item_count);

        let player_game_data_bytes = &data[41008..41008+432]; // 5144*8..5144*8+432

        let _ = &player_game_data_bytes[0..4];

        let _ = &player_game_data_bytes[4..8];

        let hp = &player_game_data_bytes[8..12];

        let hp =  u32::from_le_bytes(
            hp.try_into().unwrap()
        );

        let max_hp = &player_game_data_bytes[12..16];

        let max_hp =  u32::from_le_bytes(
            max_hp.try_into().unwrap()
        );

        let base_max_hp = &player_game_data_bytes[16..20];

        let base_max_hp =  u32::from_le_bytes(
            base_max_hp.try_into().unwrap()
        );

        println!("Current HP: {:?} \\n Max HP: {:?} \\n Base Max HP {:?}", hp, max_hp, base_max_hp);

    }
    Ok(())
}