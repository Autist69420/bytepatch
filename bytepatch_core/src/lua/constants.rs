use scroll::{Endian, Pread};

use super::LuaString;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Constant {
    LUA_TNIL,
    LUA_TBOOLEAN(bool),
    LUA_TNUMBER(f64),
    LUA_TSTRING(String),
}

impl<'a> Constant {
    pub fn decode(
        src: &'a [u8],
        offset: &mut usize,
        endian: Endian,
    ) -> Result<Constant, Box<dyn std::error::Error>> {
        let const_type: u8 = src.gread_with(offset, endian)?;
        let constant = match const_type {
            0 => Constant::LUA_TNIL,
            1 => {
                let value: u8 = src.gread_with(offset, endian)?;
                Constant::LUA_TBOOLEAN(value != 0)
            },
            3 => {
                let value: f64 = src.gread_with(offset, endian)?;
                Constant::LUA_TNUMBER(value)
            },
            4 => {
                let str = LuaString::read(src, offset, endian)?;
                Constant::LUA_TSTRING(str.into())
            },
            _ => unreachable!("Somehow got an invalid constant type")
        };

        Ok(constant)
    }
}
