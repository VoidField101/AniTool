// Copyright (c) 2022 voidfield101
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

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
