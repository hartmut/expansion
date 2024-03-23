// Experimental Simulator of a cooperative solar system economy.
// See doc/LICENSE for licensing information
//
// descriptions for entities
// https://bevy-cheatbook.github.io/fundamentals/time.html
use self::super::config::Config;
use crate::core::common::fileoperations::*;
use hifitime::Epoch;
use bevy::prelude::*;
use ron::ser::{to_writer_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

// Descriptions
#[derive(Resource, Debug, Serialize, Deserialize)]
pub struct Worldtime {
    pub epoch: Epoch, // Epoch from hifitime for orbital calculations
    pub tick_counter: u64, // counter of current step
    pub warp: u64,         // speedup of world- vs. real-time
    pub time_last: SystemTime, // last time in realtime a step has been taken
    pub step_leng: Duration,   // duration between two steps in realtime
    pub step_leng_warp: Duration, // duration between two steps in worldtime
}

impl Worldtime {
    pub fn default() -> Worldtime {
        Worldtime {
            epoch: Epoch::from_gregorian_utc(2030,1,1,0,0,0,0), 
            tick_counter: 1,
            warp: 3600,
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
        // TODO replace with better loader and error handling
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
        
        // initialize from file
        let mut worldtime = Worldtime::load_config(
            "assets/saves/resources/worldtime.ron".to_string(),
            config.get_tick_length(),
        );

        // initialize from default
        // let mut worldtime = Worldtime::default();
        
        // reset timer
        worldtime.time_last = SystemTime::now();
        worldtime.step_leng = Duration::new(0, 0);
        worldtime.step_leng_warp = Duration::new(0, 0);
        worldtime
    }
}
