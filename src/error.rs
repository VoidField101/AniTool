// Copyright (c) 2022 voidfield101
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

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
    },

    #[error("Parsing error at offset {offset:?}. {context:?}")]
    GenericParsingError {
        offset: u64,
        context: String
    }
}