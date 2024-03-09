use scroll::{Endian, Pread};

use crate::try_gread_vec_with;

#[derive(Debug)]
pub struct SourceLines(pub Vec<u32>);

impl<'a> SourceLines {
    pub fn read(
        src: &'a [u8],
        offset: &mut usize,
        endian: Endian,
    ) -> Result<SourceLines, Box<dyn std::error::Error>> {
        let amount: u32 = src.gread_with(offset, endian)?;
        let lines: Vec<u32> = try_gread_vec_with!(src, offset, amount, endian);

        Ok(SourceLines(lines))
    }
}
