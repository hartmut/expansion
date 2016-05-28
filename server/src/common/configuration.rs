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
pub struct Configuration {
    tick: u64,
    filename_player: String,
    filename_station: String,
    filename_module: String,
}

impl Configuration {

    pub fn load_config() -> Configuration {

        // Create a path to the desired file
        let path = Path::new("server/src/data/config.toml");
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
            Some(toml) => toml,
            None => {
                for err in &parser.errors{
                    let (loline, locol) = parser.to_linecol(err.lo);
                    let (hiline, hicol) = parser.to_linecol(err.hi);
                    println!("{}:{}:{}-{}:{} error: {}",
                             display, loline, locol, hiline, hicol, err.desc);
                 }
                 //TODO doesn't compile
                 return
            },
        };

//        let json = convert(Value::Table(toml));
//        println!("{}", json.pretty());
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

// fn convert(toml: Value) -> Json {
//     match toml {
//         Value::String(s) => Json::String(s),
//         Value::Integer(i) => Json::I64(i),
//         Value::Float(f) => Json::F64(f),
//         Value::Boolean(b) => Json::Boolean(b),
//         Value::Array(arr) => Json::Array(arr.into_iter().map(convert).collect()),
//         Value::Table(table) => Json::Object(table.into_iter().map(|(k, v)| {
//             (k, convert(v))
//         }).collect()),
//         Value::Datetime(dt) => Json::String(dt),
//     }
// }
