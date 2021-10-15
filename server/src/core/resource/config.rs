// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
// Old configuration - will be reworked

// implemented with default - will obsolete configuration.rs
use serde::Deserialize;
/// for initalization and configuration

/// used mods
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use bevy::log::prelude::*;
use toml;

/// When you add or remove a parameter you need to change
/// - the config.toml file
/// - the structures ConfigWrap and Config
/// - the implementation of default
/// - the implementation of the function integrate_loaded_config
/// - and add or remove a function to get the value
// change to prefab from ron

#[derive(Debug, Deserialize, Clone)]
enum StorageType {
    File,
}

impl Default for StorageType {
    fn default() -> Self {
        StorageType::File
    }
}

/// Configuration
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    tick_length: u64,
    o2_per_person: u64,
    food_per_person: f32,
    water_per_person: f32,
    structure: FileData,
    player: FileData,
    module: FileData,
    elements: FileData,
    components: FileData,
}

/// configuration
#[derive(Debug, Deserialize, Clone)]
struct ConfigWrap {
    tick_length: Option<u64>,
    o2_per_person: Option<u64>,
    food_per_person: Option<f32>,
    water_per_person: Option<f32>,
    structure: Option<FileDataWrap>,
    player: Option<FileDataWrap>,
    module: Option<FileDataWrap>,
    elements: Option<FileDataWrap>,
    components: Option<FileDataWrap>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct FileData {
    storage_method: StorageType,
    datafile: String,
    source: String,
}

#[derive(Debug, Deserialize, Clone)]
struct FileDataWrap {
    pub storage_method: Option<String>,
    pub datafile: Option<String>,
    pub source: Option<String>,
}

// after changing this you also need to change structure ConfigWrap and the
// function Config.integrate_loaded_config
impl Default for Config {
    fn default() -> Config {
        let mut config = Config {
            // default tick length in world time is 6 hours
            tick_length: 6,
            // default 02 need per person and 6 hours is 150 liters
            o2_per_person: 150,
            // 1/2 kg / day and person hydrated
            food_per_person: 0.5,
            // 5 l / day and person
            water_per_person: 5.,
            structure: FileData::new_file("resources/station.json"),
            player: FileData::new_file("resources/player.json"),
            module: FileData::new_file("resources/module.json"),
            elements: FileData::new_file("resources/PeriodicTableJSON-cleaned.json"),
            components: FileData::new_file("resources/components.json"),
        };

        // configuration is here server/src/resources/config.toml
        let file = "resources/config.toml";
        let path = Path::new(file);
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

        let decoded: ConfigWrap = toml::de::from_str(&input).unwrap();
        config.integrate_loaded_config(decoded);
        info!("Config initialized");
        config
    }
}

impl FileData {
    pub fn new_file(file: &str) -> Self {
        FileData {
            storage_method: StorageType::File,
            datafile: file.to_string(),
            source: "".to_string(),
        }
    }

    pub fn get_datafile(&self) -> String {
        self.datafile.clone()
    }
}

impl FileDataWrap {
    fn extract(&self) -> FileData {
        let mut filedata = FileData::default();
        if let Some(_x) = &self.storage_method {
            filedata.storage_method = StorageType::File
        };
        if let Some(x) = &self.datafile {
            filedata.datafile = x.clone()
        };
        if let Some(x) = &self.source {
            filedata.source = x.clone()
        };

        filedata
    }
}

impl Config {
    pub fn load_config(file: &str) -> Config {
        // configuration is here server/src/resources/config.toml
        let path = Path::new(file);
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

        let decoded: ConfigWrap = toml::de::from_str(&input).unwrap();
        let mut config: Config = Default::default();
        config.integrate_loaded_config(decoded);

        config
    }

    fn integrate_loaded_config(&mut self, input: ConfigWrap) {
        if let Some(x) = input.tick_length {
            self.tick_length = x
        };
        if let Some(x) = input.o2_per_person {
            self.o2_per_person = x
        };
        if let Some(x) = input.food_per_person {
            self.food_per_person = x
        };
        if let Some(x) = input.water_per_person {
            self.water_per_person = x
        };
        if let Some(x) = input.elements {
            self.elements = x.extract();
        };
        // TODO more files need to be intergrated
    }

    pub fn get_tick_length(&self) -> u64 {
        self.tick_length
    }

    pub fn get_o2(&self) -> u64 {
        self.o2_per_person
    }

    pub fn get_food(&self) -> f32 {
        self.food_per_person
    }

    pub fn get_water(&self) -> f32 {
        self.water_per_person
    }

    pub fn get_structure_config(&self) -> FileData {
        self.structure.clone()
    }

    pub fn get_player_config(&self) -> FileData {
        self.player.clone()
    }

    pub fn get_module_config(&self) -> FileData {
        self.module.clone()
    }

    pub fn get_elements_config(&self) -> FileData {
        self.elements.clone()
    }

    pub fn get_components_config(&self) -> FileData {
        self.components.clone()
    }
}
