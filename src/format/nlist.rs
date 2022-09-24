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

use byteorder::{ReadBytesExt, LittleEndian};

use crate::error::ParsingError;

use super::RiffParsable;

/**
 * Contains the structure to store a generic DWORD (u32) list in Riff files.
 * Used for sequence and rate data
 */
#[derive(Debug)]
pub struct NListChunk {
    list: Vec<u32>,
    size: u32
}

impl RiffParsable for NListChunk {
    fn parse(input: &mut dyn std::io::Read, size: u32, offset: u64, parent: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if size % 4 != 0 {
            return Err(Box::new(ParsingError::InvalidDataError { chunk: parent.to_string(), offset: offset-4, context: "Header size has to be divisible by 4".to_string() }));
        }
        
        let len = size / 4;
        let mut list = Vec::new();

        for _ in 0..len {
            list.push(input.read_u32::<LittleEndian>()?);
        }

        Ok(Self {
            list: list,
            size: size
        })
    }

    fn size(&self) -> u32 {
        self.size
    }
}