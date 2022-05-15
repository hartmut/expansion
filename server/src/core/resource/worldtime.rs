// Experimental Simulator of a cooperative solar system economy.
// See doc/LICENSE for licensing information
//
// descriptions for entities
use self::super::config::Config;
use crate::core::common::fileoperations::*;
use bevy::prelude::*;
use chrono;
use ron::ser::{to_writer_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

// Descriptions
// TODO express time without crate "chrono"
#[derive(Debug, Serialize, Deserialize)]
pub struct Worldtime {
    pub tick_counter: u64,                                // counter of current step
    pub warp: u64,                                        // speedup of world- vs. real-time
    pub worldtime: chrono::DateTime<chrono::FixedOffset>, // worldtime in date format
    pub time_last: SystemTime, // last time in realtime a step has been taken
    pub step_leng: Duration,   // duration between two steps in realtime
    pub step_leng_warp: Duration, // duration between two steps in worldtime
}

impl Worldtime {
    pub fn default() -> Worldtime {
        Worldtime {
            tick_counter: 1,
            warp: 3600,
            worldtime: chrono::DateTime::parse_from_rfc2822("1 Jan 2030 00:00:00 +0000").unwrap(),
            time_last: SystemTime::now(),
            step_leng: Duration::new(0, 0),
            step_leng_warp: Duration::new(0, 0),
        }
    }

    pub fn save_config(&self, pretty: PrettyConfig) {
        let ron_buffer = create_file_writer("assets/saves/resources/worldtime.ron").unwrap();
        let r = to_writer_pretty(ron_buffer, &self, pretty);
        if r.is_err() {
            println!("Serialization failed for worldtime");
        }
    }

    pub fn load_config(file: String, tick_length: u64) -> Worldtime {
        let ronconfig = read_file_to_string(file);
        let mut worldtime = Worldtime::default();
        if !ronconfig.is_empty() {
            worldtime = ron::de::from_str(&ronconfig).unwrap();
        }
        worldtime.warp = tick_length * 3600;
        worldtime
    }
}

impl FromWorld for Worldtime {
    fn from_world(world: &mut World) -> Self {
        let config = world.get_resource::<Config>().unwrap();
        info!("init worldtime");
        let mut worldtime = Worldtime::load_config(
            "assets/saves/resources/worldtime.ron".to_string(),
            config.get_tick_length(),
        );
        // reset timer
        worldtime.time_last = SystemTime::now();
        worldtime.step_leng = Duration::new(0, 0);
        worldtime
    }
}
