// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
/// for initalization and configuration

/// used mods
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::path::Path;
use std::collections::BTreeMap;
use toml::{Value, Parser};

/// configuration
#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    tick: u64,
    filename_player: String,
    filename_station: String,
    filename_module: String,
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
            Err(why) => panic!("couldn't open {}: {}", display,
                                                       why.description()),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        match file.read_to_string(&mut input) {
            Err(why) => panic!("couldn't read {}: {}", display,
                                                       why.description()),
            Ok(_) => print!("{} contains:\n{}\n\n", display, input),
        }

        let mut parser = Parser::new(&input);

        let toml = match parser.parse() {
            None       =>
                {
                    for err in &parser.errors {
                        let (loline, locol) = parser.to_linecol(err.lo);
                        let (hiline, hicol) = parser.to_linecol(err.hi);
                        println!("{}:{}:{}-{}:{} error: {}",
                                 display, loline, locol, hiline, hicol, err.desc)
                     }
                     panic!("configuration error");
                },
            Some(toml) => toml,
        };

        // decompose the toml file.
        // IDEA: try to interpret the toml file in a less complex way
        let conf:BTreeMap<String, Value> = toml;
        let global = conf.get(&"global".to_string()).unwrap();
        let player = conf.get(&"playerdata".to_string()).unwrap();
        let station = conf.get(&"structuredata".to_string()).unwrap();
        let module = conf.get(&"moduledata".to_string()).unwrap();

        // create the Configuration structure
        Configuration {
            tick:  global.lookup("tick").unwrap().as_integer().unwrap() as u64,
            filename_player:  player.lookup("datafile").unwrap().as_str().unwrap().to_string(),
            filename_station: station.lookup("datafile_station").unwrap().as_str().unwrap().to_string(),
            filename_module:  module.lookup("datafile").unwrap().as_str().unwrap().to_string(),
        }
    }

    pub fn get_tick(&self) -> u64 {
        self.tick
    }

    pub fn get_filenameplayer(&self) -> String {
        self.filename_player.clone()
    }

    pub fn get_filenamestructure(&self) -> String {
        self.filename_station.clone()
    }

    pub fn get_filenamemodule(&self) -> String {
        self.filename_module.clone()
    }

}
