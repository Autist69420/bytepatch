use scroll::{Endian, Pread};

use super::LuaString;

#[derive(Debug)]
pub struct Locals(pub Vec<Local>);

#[derive(Debug)]
pub struct Local {
    pub name: String,
    pub start_variable_scope: u32,
    pub end_variable_scope: u32,
}

impl<'a> Locals {
    pub fn read(
        src: &'a [u8],
        offset: &mut usize,
        endian: Endian,
    ) -> Result<Locals, Box<dyn std::error::Error>> {
        let amount: u32 = src.gread_with(offset, endian)?;
        let mut locals: Vec<Local> = Vec::new();
        for _ in 0..amount {
            let local = Local::read(src, offset, endian)?;
            locals.push(local);
        }

        Ok(Locals(locals))
    }
}

impl<'a> Local {
    pub fn read(
        src: &'a [u8],
        offset: &mut usize,
        endian: Endian,
    ) -> Result<Local, Box<dyn std::error::Error>> {
        let name = LuaString::read(src, offset, endian)?;
        let start_variable_scope: u32 = src.gread_with(offset, endian)?;
        let end_variable_scope: u32 = src.gread_with(offset, endian)?;

        Ok(Local {
            name: name.into(),
            start_variable_scope,
            end_variable_scope,
        })
    }
}
