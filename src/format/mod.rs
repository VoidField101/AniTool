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

use std::{error::Error, io::Read};

pub mod anih;
pub mod header;
pub mod nlist;

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