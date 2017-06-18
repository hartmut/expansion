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
    structure: Option<FileDataWrap>,
    player: Option<FileDataWrap>,
    module: Option<FileDataWrap>,
    elements: Option<FileDataWrap>,
    components: Option<FileDataWrap>,
}

// TODO implement functions for this structure
#[derive(Debug, Deserialize,Clone)]
struct FileDataWrap {
    storageMethod: Option<String>,
    datafile: Option<String>,
    source: Option<String>,
}

#[derive(Debug, Deserialize,Clone)]
pub struct FileData {
    storageMethod: String,
    datafile: String,
    source: String,
}


impl FileDataWrap {
    pub fn inner_unwrap(self) -> FileData {
        FileData {
            storageMethod: match self.storageMethod {
                Some(x) => x,
                None => "".to_string(),
            },
            datafile: match self.datafile {
                Some(x) => x,
                None => "".to_string(),
            },
            source: match self.source {
                Some(x) => x,
                None => "".to_string(),
            },
        }
    }
}

impl FileData {
    pub fn get_datafile(&self) -> String {
        self.datafile.clone()
    }
}

impl Configuration {
    pub fn load_config(args: Vec<String>) -> Configuration {

        // TODO use fileoperations from common::fileoperations
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
            Some(x) => x.clone(),
            None => 60,
        }
    }

    pub fn get_o2(&self) -> u64 {
        match self.o2_per_person {
            Some(o2) => o2.clone(),
            None => 150,
        }
    }

    pub fn get_structure_config(&self) -> FileData {
        match self.structure.clone() {
            Some(struc) => struc.inner_unwrap(),
            None => {
                FileData {
                    storageMethod: "File".to_string(),
                    datafile: "src/data/station.json".to_string(),
                    source: "".to_string(),
                }
            }
        }
    }

    pub fn get_player_config(&self) -> FileData {
        match self.player.clone() {
            Some(struc) => struc.inner_unwrap(),
            None => {
                FileData {
                    storageMethod: "File".to_string(),
                    datafile: "src/data/player.json".to_string(),
                    source: "".to_string(),
                }
            }
        }
    }

    pub fn get_module_config(&self) -> FileData {
        match self.module.clone() {
            Some(struc) => struc.inner_unwrap(),
            None => {
                FileData {
                    storageMethod: "File".to_string(),
                    datafile: "src/data/module.json".to_string(),
                    source: "".to_string(),
                }
            }
        }
    }

    pub fn get_elements_config(&self) -> FileData {
        match self.elements.clone() {
            Some(struc) => struc.inner_unwrap(),
            None => {
                FileData {
                    storageMethod: "File".to_string(),
                    datafile: "src/data/PeriodicTableJSON-cleaned.json".to_string(),
                    source: "".to_string(),
                }
            }
        }
    }

    pub fn get_components_config(&self) -> FileData {
        match self.components.clone() {
            Some(struc) => struc.inner_unwrap(),
            None => {
                FileData {
                    storageMethod: "File".to_string(),
                    datafile: "src/data/components.json".to_string(),
                    source: "".to_string(),
                }
            }
        }
    }
}

#[test]
pub fn empty_config() {
    let json = "".to_string();
    let decoded: Configuration = toml::de::from_str(&json).unwrap();
}
