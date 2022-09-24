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

use crate::format::anih::AnihChunk;
use crate::format::nlist::NListChunk;
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
    parse_riffacon(reader, 12, header.get_length() - 4)?;

    Ok(())
}


fn parse_riffacon(reader: &mut (impl Read + Seek), mut offset: u64, mut size: u32) -> Result<(), Box<dyn Error>> {
    
    while size > 0 {
        let header = RiffHeader::parse(reader, 8, offset, "RIFF")?;
        offset += header.size() as u64;
        size -= header.size() as u32;
        
        match header.get_name().unwrap_or("".to_string()).as_str() {
            "anih" => {
                let ani = AnihChunk::parse(reader, header.get_length(), offset, "anih")?;
                println!("{:?}", ani);
            },
            name @ ("rate" | "seq ") => {
                let ani = NListChunk::parse(reader, header.get_length(), offset, name)?;
                println!("{} = {:?}", name, ani);
            },
            "LIST" => {
                parse_list(reader, offset, header.get_length())?;
            }
            name => {
                println!("Warning: Unknown chunk '{name}' at {offset}");
            } 
        }

        offset += header.get_length() as u64;
        size -= header.get_length() as u32;
        //if size > 0 {
         //   reader.seek(std::io::SeekFrom::Start(offset))?;
        //}
    }

    Ok(())
}


fn parse_list(reader: &mut dyn Read, mut offset: u64, mut size: u32) -> Result<(), Box<dyn Error>> {
    let mut header_id: [u8;4] = [0;4];
    reader.read_exact(&mut header_id)?;
    
    if header_id != "fram".as_bytes() {
        return Err(Box::new(InvalidDataError{
            chunk: "LIST".to_string(),
            offset: offset,
            context: "LIST header is not of type \"fram\"".to_string(),
        }));
    }

    offset += 4;
    size -= 4;

    
    println!("LIST {:?} {:?}", size, String::from_utf8(header_id.to_vec()));

    let mut iconindex = 0;

    while size > 0 {
        let header = RiffHeader::parse(reader, 8, offset, "LIST")?;
        offset += header.size() as u64;
        size -= header.size() as u32;

        if header.get_name_bytes() != "icon".as_bytes() {
            return Err(Box::new(UnknownChunkError {
                chunk: header.get_name().unwrap_or("Unknown".to_string()),
                offset: offset,
            }));
        }
        println!("icon {}", iconindex);
        iconindex+=1;

        let mut buffer_vec: Vec<u8> = Vec::with_capacity(header.get_length().try_into().unwrap());
        reader.take(header.get_length().into()).read_to_end(&mut buffer_vec)?;

        offset += header.get_length() as u64;
        size -= header.get_length() as u32;
    }
    

    return Ok(());
}