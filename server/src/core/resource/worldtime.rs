// Experimental Simulator of a cooperative solar system economy.
// See doc/LICENSE for licensing information
//
// descriptions for entities
use chrono;
use std::time::{Duration, SystemTime};
use self::super::config::Config;
use bevy::prelude::*;

// TODO epoche calculation f64

// Descriptions
#[derive(Debug)]
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
}

impl FromWorld for Worldtime {
    fn from_world(world: &mut World) -> Self {
        let config = world.get_resource::<Config>().unwrap();
        info!("init worldtime");
        Worldtime::new(config.get_tick_length())
    }
}