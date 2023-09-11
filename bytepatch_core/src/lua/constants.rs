use scroll::{Endian, Pread};

use super::LuaString;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Constant {
    LUA_TNIL,
    LUA_TBOOLEAN(bool),
    LUA_TNUMBER(u32),
    LUA_TSTRING(LuaString),
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
                let value: u32 = src.gread_with(offset, endian)?;
                Constant::LUA_TNUMBER(value)
            },
            4 => {
                #[cfg(target_arch="x86")]
                let str = LuaString::read_u32(src, offset, endian)?;
                
                #[cfg(target_arch="x86_64")]
                let str = LuaString::read_u64(src, offset, endian)?;

                Constant::LUA_TSTRING(str)
            },
            _ => unreachable!("Somehow got an invalid constant type")
        };

        Ok(constant)
    }
}
