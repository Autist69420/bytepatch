use std::fs::File;
use std::io::{BufReader, Read};

use bytepatch_core::lua::Bytecode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open("bytecode.bin")?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let offset = &mut 0;
    let bytecode = Bytecode::read(&buffer, offset, scroll::LE)?;
    // bytecode.chunk.source_name
    println!("{:#?}", bytecode);

    Ok(())
}
