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

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParsingError {

    #[error("Chunk header type '{chunk:?}' at {offset:?} unknown")]
    UnknownChunkError {
        chunk: String,
        offset: u64
    },

    #[error("Chunk '{chunk:?}' at offset {offset:?} contains invalid values. {context:?}")]
    InvalidDataError {
        chunk: String,
        offset: u64,
        context: String
    }
}