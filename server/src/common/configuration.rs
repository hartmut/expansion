// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
/// for initalization and configuration

/// used mods
use std::fs::File;
// use std::env;
// use std::io;
use std::io::prelude::*;
use std::error::Error;
use std::path::Path;
use std::collections::BTreeMap;

use toml::{Value, Parser};
// use serde_json;

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

        let mut toml = match parser.parse() {
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


        // just for debugging purposes
        let mut conf:BTreeMap<String, Value> = toml;
        println!("{:?} \n", conf);

        // TODO extract values from toml table
        let mut global = conf.get(&"global".to_string()).unwrap().type_str();
        // let mut global:BTreeMap<String, Value> = globalval;
        println!("{:?}", global);

        // continue
        let mut player = conf.get(&"playerdata".to_string());
        let mut station = conf.get(&"stationdata".to_string());
        let mut module = conf.get(&"moduledata".to_string());

        // let out = Configuration {tick: global.get(&"tick".to_string()),
        let out = Configuration {tick:  2,
                                filename_player:  "a".to_string(),
                                filename_station: "a".to_string(),
                                filename_module:  "a".to_string(),
                            };
        out
    }

    pub fn get_tick(&self) -> u64 {
        self.tick
    }
}
