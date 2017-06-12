// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
/// for initalization and configuration

/// used mods
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::path::Path;
use toml;
use common::fileoperations::*;

/// configuration
#[derive(Debug, Deserialize,Clone)]
pub struct Configuration {
    tick: Option<u64>,
    o2_per_person: Option<u64>,
    structure: Option<FileData>,
    player: Option<FileData>,
    module: Option<FileData>,
    elements: Option<FileData>,
    components: Option<FileData>,
}

// TODO implement functions for this structure
#[derive(Debug, Deserialize,Clone)]
struct FileData {
    storagemethod: Option<String>,
    datafile: Option<String>,
    source: Option<String>,
}

impl Configuration {
    pub fn load_config(args: Vec<String>) -> Configuration {

        // configuration is here server/src/data/config.toml
        let path = Path::new(&args[1]);
        let display = path.display();
        let mut input = String::new();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open {}: {}", display, why.description()),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        match file.read_to_string(&mut input) {
            Err(why) => panic!("couldn't read {}: {}", display, why.description()),
            // Ok(_) => print!("{} contains:\n{}\n\n", display, input),
            Ok(_) => print!(""),
        }

        let decoded: Configuration = toml::de::from_str(&input).unwrap();
        decoded
    }

    pub fn get_tick(&self) -> u64 {
        // self.tick
        match self.tick {
            Some(x) => x,
            None => 2,
        }
    }
    // TODO rewrite functions
    pub fn get_o2(&self) -> u64 {
        // self.o2_per_person.clone()
        0
    }

    pub fn get_filenameplayer(&self) -> String {
        // self.filename_player.clone()
        "".to_string()
    }

    pub fn get_filenamestation(&self) -> String {
        // self.filename_station.clone()
        "".to_string()
    }

    pub fn get_filenamemodule(&self) -> String {
        // self.filename_module.clone()
        "".to_string()
    }

    pub fn get_filenameelements(&self) -> String {
        // self.filename_elements.clone()
        "".to_string()
    }

    pub fn get_filenamecomponents(&self) -> String {
        // self.filename_components.clone()
        "".to_string()
    }
}
