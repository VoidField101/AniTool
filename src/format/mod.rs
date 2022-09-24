// Copyright (c) 2022 voidfield101
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{error::Error, io::Read};

pub mod header;

/**
 * Trait implemented by all structs that are parsed from a Riff file.
 */
pub trait RiffParsable {

    /**
     * Parses the next data in the reader.
     * size is the maximum amount of data that should be read (determined by the header or 8 for headers)
     * offset is used for logging and contains the current position in the file
     * parent is the parent header name (header name for data parsing, parent header for headers and empty for RIFF)
     */
    fn parse(input: &mut dyn Read, size: u32, offset: u64, parent: &str) -> Result<Self, Box<dyn Error>> 
        where Self: Sized;

    /**
     * Size of the structure (Fixed to 8 for headers)
     */
    fn size(&self) -> u32;
}