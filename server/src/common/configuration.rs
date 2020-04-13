// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
use serde::Deserialize;
/// for initalization and configuration

/// used mods
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use toml;

/// configuration
#[derive(Debug, Deserialize, Clone)]
pub struct Configuration {
    tick: Option<u64>,
    tick_length: Option<u64>,
    o2_per_person: Option<u64>,
    food_per_person: Option<u64>,
    water_per_person: Option<u64>,
    structure: Option<FileDataWrap>,
    player: Option<FileDataWrap>,
    module: Option<FileDataWrap>,
    elements: Option<FileDataWrap>,
    components: Option<FileDataWrap>,
}

// TODO implement functions for this structure
#[derive(Debug, Deserialize, Clone)]
struct FileDataWrap {
    storage_method: Option<String>,
    datafile: Option<String>,
    source: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FileData {
    storage_method: String,
    datafile: String,
    source: String,
}

impl FileDataWrap {
    pub fn inner_unwrap(self) -> FileData {
        FileData {
            // storage_method: match self.storage_method {
            //     Some(x) => x,
            //     None => "".to_string(),
            // },
            // datafile: match self.datafile {
            //     Some(x) => x,
            //     None => "".to_string(),
            // },
            // source: match self.source {
            //     Some(x) => x,
            //     None => "".to_string(),
            // },
            storage_method: self.storage_method.unwrap_or("".to_string()),
            datafile: self.datafile.unwrap_or("".to_string()),
            source: self.source.unwrap_or("".to_string()),
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
            Err(why) => panic!("couldn't open {}: {}", display, why.to_string()),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        match file.read_to_string(&mut input) {
            Err(why) => panic!("couldn't read {}: {}", display, why.to_string()),
            // Ok(_) => print!("{} contains:\n{}\n\n", display, input),
            Ok(_) => print!(""),
        }

        let decoded: Configuration = toml::de::from_str(&input).unwrap();
        decoded
    }

    // default waiting for new step in realtime is 10 seconds
    pub fn get_tick(&self) -> u64 {
        self.tick.unwrap_or(10)
    }

    // default tick length in world time is 6 hours
    pub fn get_tick_length(&self) -> u64 {
        self.tick_length.unwrap_or(6)
    }

    // default 02 need per person and 6 hours is 150 liters
    pub fn get_o2(&self) -> u64 {
        self.o2_per_person.unwrap_or(150)
        // match self.o2_per_person {
        //     Some(o2) => o2.clone(),
        //     None => 150,
        // }
    }

    pub fn get_food(&self) -> u64 {
        match self.food_per_person {
            Some(food) => food.clone(),
            None => 1,
        }
    }

    pub fn get_water(&self) -> u64 {
        match self.water_per_person {
            Some(water) => water.clone(),
            None => 5,
        }
    }

    pub fn get_structure_config(&self) -> FileData {
        match self.structure.clone() {
            Some(struc) => struc.inner_unwrap(),
            None => FileData {
                storage_method: "File".to_string(),
                datafile: "data/station.json".to_string(),
                source: "".to_string(),
            },
        }
    }

    pub fn get_player_config(&self) -> FileData {
        match self.player.clone() {
            Some(struc) => struc.inner_unwrap(),
            None => FileData {
                storage_method: "File".to_string(),
                datafile: "data/player.json".to_string(),
                source: "".to_string(),
            },
        }
    }

    pub fn get_module_config(&self) -> FileData {
        match self.module.clone() {
            Some(struc) => struc.inner_unwrap(),
            None => FileData {
                storage_method: "File".to_string(),
                datafile: "data/module.json".to_string(),
                source: "".to_string(),
            },
        }
    }

    pub fn get_elements_config(&self) -> FileData {
        match self.elements.clone() {
            Some(struc) => struc.inner_unwrap(),
            None => FileData {
                storage_method: "File".to_string(),
                datafile: "data/PeriodicTableJSON-cleaned.json".to_string(),
                source: "".to_string(),
            },
        }
    }

    pub fn get_components_config(&self) -> FileData {
        match self.components.clone() {
            Some(struc) => struc.inner_unwrap(),
            None => FileData {
                storage_method: "File".to_string(),
                datafile: "data/components.json".to_string(),
                source: "".to_string(),
            },
        }
    }
}

#[test]
pub fn empty_config() {
    let json = "".to_string();
    let _decoded: Configuration = toml::de::from_str(&json).unwrap();
    //TODO test missing
}
