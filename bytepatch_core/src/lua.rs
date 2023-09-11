use std::fmt::Display;

use scroll::{ctx, Endian, Pread};

pub mod instructions;
pub mod constants;

use crate::try_gread_vec_with;

use instructions::{Instruction, Opcode};

#[derive(Debug)]
pub struct Header {
    pub magic: u32,           // four bytes
    pub version: u8,          // one byte, Version number, 0x51 (81 decimal) for Lua 5.1
    pub format_version: u8,   // one byte
    pub endianess_flag: u8,   // one byte, default is 1,  0=big endian, 1=little endian
    pub int_size: u8,         // one byte, default value is 4, Size of int (in bytes)
    pub size_t_size: u8,      // one byte default value is 4, Size of size_t (in bytes)
    pub instruction_size: u8, // one byte, default value is 4, Size of Instruction (in bytes)
    pub lua_number_size: u8,  // one byte, default value is 8, Size of lua_Number (in bytes)
    pub integral_flag: u8,    // one byte default value 0, 0=floating-point, 1=integral number type
}

impl<'a> ctx::TryFromCtx<'a, Endian> for Header {
    type Error = scroll::Error;
    fn try_from_ctx(src: &'a [u8], endian: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let magic: u32 = src.gread_with(offset, endian)?;
        let version: u8 = src.gread_with(offset, endian)?;
        let format_version: u8 = src.gread_with(offset, endian)?;
        let endianess_flag: u8 = src.gread_with(offset, endian)?;
        let int_size: u8 = src.gread_with(offset, endian)?;
        let size_t_size: u8 = src.gread_with(offset, endian)?;
        let instruction_size: u8 = src.gread_with(offset, endian)?;
        let lua_number_size: u8 = src.gread_with(offset, endian)?;
        let integral_flag: u8 = src.gread_with(offset, endian)?;

        Ok((
            Header {
                magic,
                version,
                format_version,
                endianess_flag,
                int_size,
                size_t_size,
                instruction_size,
                lua_number_size,
                integral_flag,
            },
            *offset,
        ))
    }
}

#[derive(Debug)]
pub struct LuaString(Vec<u8>);

impl Into<String> for LuaString {
    fn into(self) -> String {
        let str = String::from_utf8(self.0).unwrap();
        str
    }
}

impl Display for LuaString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = String::from_utf8(self.0.clone()).unwrap();
        f.write_str(&str)
    }
}

impl<'a> LuaString {
    pub fn read_u32(
        src: &'a [u8],
        offset: &mut usize,
        endian: Endian,
    ) -> Result<LuaString, Box<dyn std::error::Error>> {
        let size: u32 = src.gread_with(offset, endian)?;
        let data: Vec<u8> = try_gread_vec_with!(src, offset, size, endian);

        Ok(LuaString(data))
    }

    pub fn read_u64(
        src: &'a [u8],
        offset: &mut usize,
        endian: Endian,
    ) -> Result<LuaString, Box<dyn std::error::Error>> {
        let size: u64 = src.gread_with(offset, endian)?;
        let data: Vec<u8> = try_gread_vec_with!(src, offset, size, endian);

        Ok(LuaString(data))
    }
}

#[derive(Debug)]
pub struct Instructions(Vec<Instruction>);

impl<'a> Instructions {
    pub fn read(
        src: &'a [u8],
        offset: &mut usize,
        endian: Endian,
    ) -> Result<Instructions, Box<dyn std::error::Error>> {
        let amount: u32 = src.gread_with(offset, endian)?;
        let instruction_list: Vec<u32> = try_gread_vec_with!(src, offset, amount, endian);
        let instructions: Vec<Instruction> = instruction_list.iter().map(|f| Opcode::decode(*f)).collect();

        Ok(Instructions (instructions))
    }
}

#[derive(Debug)]
pub struct Chunk {
    pub source_name: LuaString,
    // We hope this is right all the time, if not, fuck you!
    pub line_defined: u32,
    pub last_line_defined: u32,
    pub num_upvalues: u8,
    pub num_params: u8,
    pub is_vararg: u8,
    pub max_stack_size: u8,
    pub instructions: Vec<Instruction>
}

impl<'a> Chunk {
    pub fn read_u32(
        src: &'a [u8],
        offset: &mut usize,
        endian: Endian,
    ) -> Result<Chunk, Box<dyn std::error::Error>> {
        let source_name = LuaString::read_u32(src, offset, endian)?;
        let line_defined: u32 = src.gread_with(offset, endian)?;
        let last_line_defined: u32 = src.gread_with(offset, endian)?;
        let num_upvalues: u8 = src.gread_with(offset, endian)?;
        let num_params: u8 = src.gread_with(offset, endian)?;
        let is_vararg: u8 = src.gread_with(offset, endian)?;
        let max_stack_size: u8 = src.gread_with(offset, endian)?;

        let instructions = Instructions::read(src, offset, endian)?;

        Ok(Chunk {
            source_name,
            line_defined,
            last_line_defined,
            num_upvalues,
            num_params,
            is_vararg,
            max_stack_size,
            instructions: instructions.0,
        })
    }

    pub fn read_u64(
        src: &'a [u8],
        offset: &mut usize,
        endian: Endian,
    ) -> Result<Chunk, Box<dyn std::error::Error>> {
        let source_name = LuaString::read_u64(src, offset, endian)?;
        let line_defined: u32 = src.gread_with(offset, endian)?;
        let last_line_defined: u32 = src.gread_with(offset, endian)?;
        let num_upvalues: u8 = src.gread_with(offset, endian)?;
        let num_params: u8 = src.gread_with(offset, endian)?;
        let is_vararg: u8 = src.gread_with(offset, endian)?;
        let max_stack_size: u8 = src.gread_with(offset, endian)?;

        let instructions = Instructions::read(src, offset, endian)?;

        Ok(Chunk {
            source_name,
            line_defined,
            last_line_defined,
            num_upvalues,
            num_params,
            is_vararg,
            max_stack_size,
            instructions: instructions.0,
        })
    }
}

#[derive(Debug)]
pub struct Bytecode {
    pub header: Header,
    pub chunk: Chunk,
}

impl<'a> Bytecode {
    pub fn read(
        src: &'a [u8],
        offset: &mut usize,
        endian: Endian,
    ) -> Result<Bytecode, Box<dyn std::error::Error>> {
        let header: Header = src.gread_with(offset, scroll::LE)?;
        let chunk = match header.size_t_size {
            4 => Chunk::read_u32(src, offset, endian)?,
            8 => Chunk::read_u64(src, offset, endian)?,
            _ => unreachable!("Invalid size_t size, expected 4 or 8 depending on arch"),
        };

        Ok(Bytecode { header, chunk })
    }
}

