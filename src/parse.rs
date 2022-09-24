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

use std::io::{Seek};
use std::{io::Read, error::Error};

use crate::format::{header::RiffHeader, RiffParsable};
use crate::error::ParsingError::{UnknownChunkError,InvalidDataError};

pub fn parse_entry(reader: &mut (impl Read + Seek)) -> Result<(), Box<dyn Error>> {
    let header = RiffHeader::parse(reader, 8, 0, "")?;

    if *header.get_name_bytes() != "RIFF".as_bytes() {
        return Err(Box::new(UnknownChunkError{
            chunk: header.get_name().unwrap_or("Unknown".to_string()),
            offset: 0,
        }));
    }

    let mut header_id: [u8;4] = [0;4];
    reader.read_exact(&mut header_id)?;

    if header_id != "ACON".as_bytes() {
        return Err(Box::new(InvalidDataError{
            chunk: "RIFF".to_string(),
            offset: 8,
            context: "RIFF header is not of type \"ACON\"".to_string(),
        }));
    }

    println!("{:?} {:?}", header, String::from_utf8(header_id.to_vec()));

    Ok(())
}
