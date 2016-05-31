// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
/// for initalization and configuration

/// used mods
use std::fs::File;
use std::env;
use std::io;
use std::io::prelude::*;
use std::error::Error;
use std::path::Path;

use toml;
use serde_json;

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
            Ok(_) => print!("{} contains:\n{}", display, input),
        }

        let mut parser = toml::Parser::new(&input);

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

        //just for debugging purposes
        println!("{:?}", toml);

        let out = Configuration {tick: 2,
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

// fn convert(toml: toml::Value) -> Vec<String> {
//     let mut output = Vec::new();
//     match toml {
//         toml::Value::String(s) => output.push(s),
//         toml::Value::Integer(i) => output.push(i),
//         toml::Value::Float(f) => output.push(f),
//         toml::Value::Boolean(b) => output.push(b),
//         toml::Value::Array(arr) => Json::Array(arr.into_iter().map(convert).collect()),
//         toml::Value::Table(table) => Json::Object(table.into_iter().map(|(k, v)| {
//             (k, convert(v))
//         }).collect()),
//         toml::Value::Datetime(dt) => Json::String(dt),
//     }
//     output
// }
