// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// descriptions for entities
use chrono;
use std::time::{Duration, SystemTime};

// Descriptions
#[derive(Debug)]
pub struct Worldtime {
    pub tick_counter: u64,                                // counter of current step
    pub tick_length: chrono::Duration,                    // duration in worldtime between two ticks
    pub tick_dur: chrono::Duration,                       // duration in realtime
    pub warp: u64,                                        // speedup of world- vs. real-time
    pub time_last: SystemTime,                            // last time a step has been taken
    pub worldtime: chrono::DateTime<chrono::FixedOffset>, // worldtime in date format
    pub step_leng: Duration, // duration between two steps in worldtime in secs
}

impl Worldtime {
    pub fn new(tick_dur: u64, tick_length: u64) -> Self {
        Worldtime {
            tick_counter: 1,
            tick_dur: chrono::Duration::seconds(tick_dur as i64),
            tick_length: chrono::Duration::hours(tick_length as i64),
            warp: (tick_length as u64) * 3600 / tick_dur,
            time_last: SystemTime::now(),
            worldtime: chrono::DateTime::parse_from_rfc2822("1 Jan 2030 00:00:00 +0000").unwrap(),
            step_leng: Duration::new(0, 0),
        }
    }

}
