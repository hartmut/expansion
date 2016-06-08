// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
/// helper for reading files

/// used mods
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::Path;
use std::error::Error;

pub fn newreader (FileName: String) -> BufReader<File> {

    let path = Path::new(&FileName);

    // Open the path in read-only mode, returns `io::Result<File>`
    let f = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", path.display(),
                                                   why.description()),
        Ok(file) => file,
    };

    BufReader::new(f)
}

pub fn printfile (f: BufReader<File>) {
    println!("\n ******************************** \n");
    for line in f.lines() {
        println!("{}", line.unwrap());
    }
    println!("\n ******************************** \n");
}
