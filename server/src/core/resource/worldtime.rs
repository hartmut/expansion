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

// TODO change to load from ron file

// Descriptions
#[derive(Debug, Serialize, Deserialize)]
pub struct Worldtime {
    pub tick_counter: u64,                                // counter of current step
    pub warp: u64,                                        // speedup of world- vs. real-time
    pub time_last: SystemTime,                            // last time a step has been taken
    pub worldtime: chrono::DateTime<chrono::FixedOffset>, // worldtime in date format
    pub step_leng: Duration, // duration between two steps in worldtime in secs
}

impl Worldtime {
    pub fn new(tick_length: u64) -> Self {
        Worldtime {
            tick_counter: 1,
            warp: (tick_length as u64) * 3600,
            time_last: SystemTime::now(),
            worldtime: chrono::DateTime::parse_from_rfc2822("1 Jan 2030 00:00:00 +0000").unwrap(),
            step_leng: Duration::new(0, 0),
        }
    }

    pub fn save_config(&self, pretty: PrettyConfig) {
        let ron_buffer = create_file_writer("assets/saves/resources/worldtime.ron").unwrap();
        let r = to_writer_pretty(ron_buffer, &self, pretty);
        if r.is_err() {
            println!("Serialization failed for worldtime");
        }
    }
}

impl FromWorld for Worldtime {
    fn from_world(world: &mut World) -> Self {
        let config = world.get_resource::<Config>().unwrap();
        info!("init worldtime");
        Worldtime::new(config.get_tick_length())
    }
}
