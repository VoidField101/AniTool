// Copyright (c) 2022 voidfield101
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::fs::File;

use parse::parse_entry;

mod format;
mod error;
mod parse;


fn main() {
    let mut file = File::open("test.ani").expect("Error");
    parse_entry(&mut file).expect("Error");
}
