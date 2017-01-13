// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
/// helper for reading files
/// tests in https://github.com/hartmut/rust-notes/tree/master/code/strings

/// used mods
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::LineWriter;
use std::path::Path;
use std::error::Error;

// IDEA rewrite to use json database by default, simpler?

// General file functions

/**
print the content of a file

# Returns

* nothing

# Arguments

* `f` File to print, expects a BufReader<File>

**/
pub fn printfile(f: BufReader<File>) {
    println!("\n ******************************** \n");
    for line in f.lines() {
        println!("{}", line.unwrap());
    }
    println!("\n ******************************** \n");
}

pub fn newreader(filename: String) -> BufReader<File> {

    let path = Path::new(&filename);

    // Open the path in read-only mode, returns `io::Result<File>`
    let f = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };

    BufReader::new(f)
}

// generic readline function
pub fn readline(mut f: &mut BufReader<File>) -> Option<String> {

    let mut line = String::new();

    if f.read_line(&mut line).unwrap() == 0 {
        None
    } else {
        let lastchar = line.pop().unwrap();

        if lastchar != '\n' {
            line.push(lastchar)
        }

        Some(line)
    }

}

// writing functions
// - pub fn newlinewriter (filename: String) -> LineWriter<File>
// - pub fn writerecord (f: &mut LineWriter<File>, output: String) -> u64
//

pub fn newlinewriter(filename: String) -> LineWriter<File> {

    let path = Path::new(&filename);

    // Open the file for writing, returns `io::Result<File>`
    let f = match File::create(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };

    LineWriter::new(f)
}

// genericline  writerecord function, returns length of written data
// TODO better error handling in writerecord
pub fn writerecord(mut f: &mut LineWriter<File>, output: &String) -> u64 {
    f.write(output.as_bytes()).unwrap() as u64;
    f.write('\n'.to_string().as_bytes()).unwrap() as u64
}

pub fn closefile(mut f: &mut LineWriter<File>) {
    let flush_result = f.flush();
}
