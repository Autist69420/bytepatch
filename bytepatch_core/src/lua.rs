use std::fmt::Display;

use scroll::{ctx, Endian, Pread};

pub mod constants;
pub mod instructions;
pub mod source_lines;
pub mod locals;
pub mod upvalues;

use crate::try_gread_vec_with;

use instructions::{Instruction, Opcode};
use constants::{Constant, Constants};
use source_lines::SourceLines;
use locals::{Local, Locals};
use upvalues::{Upvalues, Upvalue};

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

impl From<LuaString> for String {
    fn from(value: LuaString) -> Self {
        String::from_utf8(value.0).unwrap()
    }
}

impl Display for LuaString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = String::from_utf8(self.0.clone()).unwrap();
        f.write_str(&str)
    }
}

impl<'a> LuaString {
    #[cfg(target_arch="x86")]
    pub fn read(
        src: &'a [u8],
        offset: &mut usize,
        endian: Endian,
    ) -> Result<LuaString, Box<dyn std::error::Error>> {
        let size: u32 = src.gread_with(offset, endian)?;
        let mut data: Vec<u8> = try_gread_vec_with!(src, offset, size, endian);
        let _ = data.pop(); // We remove the null byte at the end, lmao!

        Ok(LuaString(data))
    }

    #[cfg(target_arch="x86_64")]
    pub fn read(
        src: &'a [u8],
        offset: &mut usize,
        endian: Endian,
    ) -> Result<LuaString, Box<dyn std::error::Error>> {
        let size: u64 = src.gread_with(offset, endian)?;
        let mut data: Vec<u8> = try_gread_vec_with!(src, offset, size, endian);
        let _ = data.pop(); // We remove the null byte at the end, lmao!

        Ok(LuaString(data))
    }
}

#[derive(Debug)]
pub struct Instructions(pub Vec<Instruction>);

impl<'a> Instructions {
    pub fn read(
        src: &'a [u8],
        offset: &mut usize,
        endian: Endian,
    ) -> Result<Instructions, Box<dyn std::error::Error>> {
        let amount: u32 = src.gread_with(offset, endian)?;
        let instruction_list: Vec<u32> = try_gread_vec_with!(src, offset, amount, endian);
        let instructions: Vec<Instruction> = instruction_list
            .iter()
            .map(|f| Opcode::decode(*f))
            .collect();

        Ok(Instructions(instructions))
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
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Constant>,
    pub prototypes: Vec<Chunk>,
    pub source_lines: Vec<u32>,
    pub locals: Vec<Local>,
    pub upvalues: Vec<Upvalue>
}

impl<'a> Chunk {
    pub fn read(
        src: &'a [u8],
        offset: &mut usize,
        endian: Endian,
    ) -> Result<Chunk, Box<dyn std::error::Error>> {
        let source_name = LuaString::read(src, offset, endian)?;
        let line_defined: u32 = src.gread_with(offset, endian)?;
        let last_line_defined: u32 = src.gread_with(offset, endian)?;
        let num_upvalues: u8 = src.gread_with(offset, endian)?;
        let num_params: u8 = src.gread_with(offset, endian)?;
        let is_vararg: u8 = src.gread_with(offset, endian)?;
        let max_stack_size: u8 = src.gread_with(offset, endian)?;

        let instructions = Instructions::read(src, offset, endian)?;

        let constants = Constants::read(src, offset, endian)?;

        // Messy but that can stay here, lmao.
        let prototype_amount: u32 = src.gread_with(offset, endian)?;
        let mut prototypes: Vec<Chunk> = Vec::new();
        for _ in 0..prototype_amount {
            let prototype = Chunk::read(src, offset, endian)?;
            prototypes.push(prototype);
        }

        let source_lines = SourceLines::read(src, offset, endian)?;
        let locals = Locals::read(src, offset, endian)?;
        let upvalues = Upvalues::read(src, offset, endian)?;

        Ok(Chunk {
            source_name,
            line_defined,
            last_line_defined,
            num_upvalues,
            num_params,
            is_vararg,
            max_stack_size,
            instructions: instructions.0,
            constants: constants.0,
            prototypes,
            source_lines: source_lines.0,
            locals: locals.0,
            upvalues: upvalues.0,
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
        let chunk = Chunk::read(src, offset, endian)?;

        Ok(Bytecode { header, chunk })
    }
}
