// Copyright (C) 2022 voidfield101
// 
// This file is part of AniTool.
// 
// AniTool is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
// 
// AniTool is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with AniTool.  If not, see <http://www.gnu.org/licenses/>.

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

impl AnihChunk {

    pub fn total_frames(&self) -> u32 {
        self.frames
    }

    pub fn animation_steps(&self) -> u32 {
        self.steps
    }

    pub fn bmp_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
    
    pub fn color_bits(&self) -> u32 {
        self.bits
    }

    pub fn image_planes(&self) -> u32 {
        self.planes
    }

    pub fn animation_rate(&self) -> u32 {
        self.rate
    }

    pub fn is_ico(&self) -> bool {
        self.ico
    }

    pub fn is_sequenced(&self) -> bool {
        self.sequenced
    }

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