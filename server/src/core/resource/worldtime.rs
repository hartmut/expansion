// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// descriptions for entities
use chrono;
use log::info;
use std::thread;
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

    // COMBAK modify to system
    pub fn step(&mut self) {
        info!("this is a step of worldtime {}", self.worldtime);

        //sleep and one more step
        thread::sleep(self.tick_dur.to_std().unwrap());
        self.tick_counter += 1;

        //update worldtime
        let time_now = SystemTime::now();
        self.step_leng = time_now
            .duration_since(self.time_last)
            .expect("SystemTime::duration_since failed");
        let step_leng_warp = self.step_leng * (self.warp as u32);
        let chrono_step_leng = chrono::Duration::from_std(step_leng_warp).unwrap();
        self.worldtime = self.worldtime + chrono_step_leng;

        // save time of this step
        self.time_last = time_now;

        info!(
            "step length with warp {:?}",
            self.step_leng * (self.warp as u32)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use specs::prelude::*;

    #[test]
    fn create_resource_worldtime() {
        let mut world = specs::World::new();
        world.insert(Worldtime::new(6, 2));
    }
}
