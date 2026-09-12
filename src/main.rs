use crate::constants::self as con;
use crate::cursor::CursorReader;
use crate::types::{Gaitem, PlayerGameData};

use std::fs::File;
use std::io::{self, BufReader, Read};

mod constants;
mod cursor;
mod types;

const CHARACTER_SLOT_OFFSETS: [u64; 1] = [0x300u64];
// ,0x280310u64,0x500320u64,0x780330u64,0xA00340u64,0xC80350u64,0xF00360u64,0x1180370u64,0x1400380u64,0x1680390u64

fn read_magic<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<[u8; 4]> {
    let mut magic = [0u8; con::MAGIC_SIZE];
    cursor.read_exact(&mut magic)?;
    Ok(magic)
}

fn read_header<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<[u8; con::PC_HEADER_SIZE]> {
    let mut header = [0u8; con::PC_HEADER_SIZE];
    cursor.read_exact(&mut header)?;
    Ok(header)
}

fn read_checksum<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<[u8; con::CHECKSUM_SIZE]> {
    let mut checksum = [0u8; con::CHECKSUM_SIZE];
    cursor.read_exact(&mut checksum)?;
    Ok(checksum)
}

fn parse_character_slot<R: Read>(
    cursor: &mut CursorReader<R>,
    save_offsets: &mut Vec<u64>,
) -> io::Result<()> {
    let mut item_count: usize = 0;

    let checksum = read_checksum(cursor)?;

    println!("Checksum: {:?}", checksum);

    save_offsets.push(cursor.position());

    let version = cursor.read_u32_le()?;

    println!("Version: {:?}", version);

    let map_id = cursor.read_u32_le()?;

    println!("Map ID: {:?}", map_id);

    cursor.discard(24)?;

    while item_count < con::GAITEM_COUNT_NEW {
        let gaitem_handle = cursor.read_u32_le()?;

        let item_id = cursor.read_u32_le()?;

        item_count += 1;

        if gaitem_handle != 0 {
            let mut _item = Gaitem {
                gaitem_handle,
                item_id,
                ..Default::default()
            };

            let handle_type = gaitem_handle & 0xF0000000;

            if gaitem_handle != 0 && handle_type != 0xC0000000 {
                let unk_0x10 = cursor.read_u32_le()?;

                _item.unk_0x10 = Some(unk_0x10);

                let unk_0x14 = cursor.read_u32_le()?;

                _item.unk_0x14 = Some(unk_0x14);

                if handle_type == 0x80000000 {
                    let gem_gaitem_handle = cursor.read_u32_le()?;

                    _item.gem_gaitem_handle = Some(gem_gaitem_handle);

                    let unk_0x1c = cursor.read_u8_le()?;

                    _item.unk_0x1c = Some(unk_0x1c);
                }
            }

            // println!("{:?}", item);
        }
    }

    println!("Item count: {:?}", item_count);

    parse_player_game_data(cursor)?;

    Ok(())
}

fn parse_player_game_data<R: Read>(cursor: &mut CursorReader<R>) -> io::Result<()> {
    let player_game_data = PlayerGameData::read(cursor)?;

    print!("{:?}", player_game_data);

    Ok(())
}

fn main() -> io::Result<()> {
    let mut save_offsets: Vec<u64> = Vec::with_capacity(10);

    // Open file and setup cursor / reader
    let file = File::open("./ER0000.sl2")?;

    let bufreader = BufReader::new(file);

    let mut cursor = crate::cursor::CursorReader::new(bufreader);

    // Read and interpret magic bytes

    let magic_bytes = read_magic(&mut cursor)?;

    match magic_bytes {
        con::MAGIC_BND4_HEX => {
            println!("Standard PC Steam Save format detected.")
        }
        con::MAGIC_SL2 => {
            println!("Alternate PC Save format detected.")
        }
        con::MAGIC_PLAYSTATION => {
            println!("Playstation Save format detected. No checksums. Output will be wrong.")
        }
        _ => {
            println!("Unknown magic bytes.")
        }
    }

    // Read Header and...ignore it :Lin:

    let header = read_header(&mut cursor)?;

    println!("Header: {:?}", header);

    for _cso in CHARACTER_SLOT_OFFSETS {
        parse_character_slot(&mut cursor, &mut save_offsets)?;
    }
    Ok(())
}
