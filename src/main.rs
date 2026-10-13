use crate::constants as con;
use crate::cursor::CursorReader;
use crate::types::{CharacterSlot, USER_DATA_10, USER_DATA_11};

use std::fs::File;
use std::io::{self, BufReader, Read};

mod constants;
mod cursor;
mod types;

const CHARACTER_SLOT_OFFSETS: [u64; 10] = [
    0x300u64,
    0x280310u64,
    0x500320u64,
    0x780330u64,
    0xA00340u64,
    0xC80350u64,
    0xF00360u64,
    0x1180370u64,
    0x1400380u64,
    0x1680390u64,
];
//

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
    let checksum = read_checksum(cursor)?;

    println!("Checksum: {:?}", checksum);

    save_offsets.push(cursor.position());

    let character_slot = Box::new(CharacterSlot::read(cursor)?);

    println!("{:?}", character_slot.player_game_data);

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

    let user_data_10 = USER_DATA_10::read(&mut cursor)?;

    USER_DATA_11::read(&mut cursor)?;

    println!("{:#?}", user_data_10);

    Ok(())
}
