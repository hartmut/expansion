// Experimental Simulator of a cooperative solar system economy.
// See doc/LICENSE for licensing information
// Old configuration - will be reworked

/// for initalization and configuration
use bevy::log::prelude::*;
use ron::ser::{to_writer_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};

/// used mods
use crate::core::common::fileoperations::*;

/// When you add or remove a parameter you need to change
/// - the config.toml file
/// - the structures ConfigWrap and Config
/// - the implementation of default
/// - the implementation of the function integrate_loaded_config
/// - and add or remove a function to get the value

#[derive(Debug, Serialize, Deserialize, Clone)]
enum StorageType {
    File,
}

impl Default for StorageType {
    fn default() -> Self {
        StorageType::File
    }
}

/// Configuration
// TODO integrate into config
// gas equivalents http://www.airproducts.com/Products/Gases/gas-facts/conversion-formulas/weight-and-volume-equivalents/oxygen.aspx
// airmix_o2 = 0.2
// airmix_n2 = 0.8

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    // one tick is tick_lenght hours in worldtime per second realtime
    tick_length: u64,
    //  needed O2 in liter per player and 6 hour tick, later value per hour
    //  https://www.quora.com/How-much-oxygen-does-the-average-person-burn-in-a-day-What-volume-of-air-is-that
    //  using the nasa values, per hour a person will need about 25l, could be higher if exercising => we will improve the model later
    o2_per_person: u64,
    // per day in kg
    food_per_person: f32,
    // per day in kg
    water_per_person: f32,
    elements: FileData,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FileData {
    storage_method: StorageType,
    datafile: String,
    source: String,
}

impl Default for Config {
    fn default() -> Config {
        let mut config = Config::new();
        let ronconfig = read_file_to_string("assets/saves/resources/config.ron".to_string());
        if !ronconfig.is_empty() {
            config = ron::de::from_str(&ronconfig).unwrap();
            info!("Config RONinitialized from file");
        }

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

impl Config {
    pub fn new() -> Config {
        Config {
            // default tick length in world time is 6 hours
            tick_length: 5,
            // default 02 need per person and 6 hours is 150 liters
            o2_per_person: 150,
            // 1/2 kg / day and person hydrated
            food_per_person: 0.5,
            // 5 l / day and person
            water_per_person: 5.,
            elements: FileData::new_file("resources/PeriodicTableJSON-cleaned.json"),
        }
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

    pub fn get_elements_config(&self) -> FileData {
        self.elements.clone()
    }

    pub fn save_config(&self, pretty: PrettyConfig) {
        let ron_buffer = create_file_writer("assets/saves/resources/config.ron").unwrap();
        let r = to_writer_pretty(ron_buffer, &self, pretty);
        if r.is_err() {
            println!("Serialization failed for configuration");
        }
    }
}
