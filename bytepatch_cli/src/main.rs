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
    println!("============== Lua Bytecode Disassembly ==============");
    println!("Chunk name: {}", bytecode.chunk.source_name);
    for instruction in bytecode.chunk.instructions {
        println!("{}", instruction);
    }

    for constant in bytecode.chunk.constants {
        println!("{:#?}", constant);
    }

    Ok(())
}
