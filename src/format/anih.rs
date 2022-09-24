// Copyright (c) 2022 voidfield101
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{io::Read, error::Error};

use byteorder::{ReadBytesExt, LittleEndian};

use crate::error::ParsingError;

use super::RiffParsable;

/**
 * Holds the Anih chunk information.
 * Anih contains generic information about the cursor (size, number of frames, etc.)
 */
#[derive(Debug)]
pub struct AnihChunk{
    frames: u32,
    steps: u32,
    width: u32,
    height: u32,
    bits: u32,
    planes: u32,
    rate: u32,

    // Flags
    ico: bool,
    sequenced: bool
}

impl RiffParsable for AnihChunk {

    fn parse(input: &mut dyn Read, size: u32, offset: u64, parent: &str) -> Result<Self, Box<dyn Error>> {
        if size != 36 {
            return Err(Box::new(ParsingError::InvalidDataError { chunk: "anih".to_string(), offset: offset-4, context: "Header size in anih has to equal 36.".to_string() }));
        }
        
        let length = input.read_u32::<LittleEndian>()?;
        
        if length != 36 {
            return Err(Box::new(ParsingError::InvalidDataError { chunk: "anih".to_string(), offset: offset, context: "Length in anih has to equal 36.".to_string() }));
        }

        let frames = input.read_u32::<LittleEndian>()?;
        let steps = input.read_u32::<LittleEndian>()?;
        let width = input.read_u32::<LittleEndian>()?;
        let height = input.read_u32::<LittleEndian>()?;
        let bits = input.read_u32::<LittleEndian>()?;
        let planes = input.read_u32::<LittleEndian>()?;
        let rate = input.read_u32::<LittleEndian>()?;
        let flags = input.read_u32::<LittleEndian>()?;

        let ico_flag = (flags & 0x01) != 0;
        let seq_flag = (flags & 0x02) != 0;

        return Ok(Self { 
            frames: frames,
            steps: steps,
            width: width,
            height: height,
            bits: bits,
            planes: planes,
            rate: rate,
            ico: ico_flag,
            sequenced: seq_flag
        });
    }

    fn size(&self) -> u32 {
        36
    }

}