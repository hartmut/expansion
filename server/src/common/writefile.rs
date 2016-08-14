// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
/// helper for reading files

/// used mods
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;
use std::error::Error;

pub fn newwriter (filename: String) -> BufReader<File> {

    let path = Path::new(&filename);

    // Open the path in read-only mode, returns `io::Result<File>`
    let f = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", path.display(),
                                                   why.description()),
        Ok(file) => file,
    };

    BufWriter::new(f)
}

// generic writeline function, returns length of written data
pub fn wirteline (f: &mut BufReader<File>, output: String) -> u64  {
    let len = f.write(&mut line).unwrap();
    f.flush();
    len
}
