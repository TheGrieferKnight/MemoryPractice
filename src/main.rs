use crate::types::EldenRingSave;

use std::fs::File;
use std::io::{self, BufReader};

mod constants;
mod cursor;
mod types;

fn main() -> io::Result<()> {
    // Open file and setup cursor / reader
    let file = File::open("./ER0000.sl2")?;

    let bufreader = BufReader::new(file);

    let mut cursor = crate::cursor::CursorReader::new(bufreader);

    let _ = EldenRingSave::read(&mut cursor)?;

    Ok(())
}
