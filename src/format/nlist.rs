// Copyright (c) 2022 voidfield101
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

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