use scroll::{Endian, Pread};

use super::LuaString;

#[derive(Debug)]
pub struct Upvalues(pub Vec<Upvalue>);

#[derive(Debug)]
pub struct Upvalue {
    pub name: String,
}

impl<'a> Upvalue {
    pub fn read(
        src: &'a [u8],
        offset: &mut usize,
        endian: Endian,
    ) -> Result<Upvalue, Box<dyn std::error::Error>> {
        let name = LuaString::read(src, offset, endian)?;

        Ok(Upvalue {
            name: name.into(),
        })
    }
}

impl<'a> Upvalues {
    pub fn read(
        src: &'a [u8],
        offset: &mut usize,
        endian: Endian,
    ) -> Result<Upvalues, Box<dyn std::error::Error>> {
        let amount: u32 = src.gread_with(offset, endian)?;
        let mut upvalues: Vec<Upvalue> = Vec::new();
        for _ in 0..amount {
            let local = Upvalue::read(src, offset, endian)?;
            upvalues.push(local);
        }

        Ok(Upvalues(upvalues))
    }
}
