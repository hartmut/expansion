// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// descriptions for entities
use specs;
use chrono;
// use std::time::Duration;
use std::thread;

// Descriptions
pub struct Worldtime {
    pub tick_counter: u64,                                // actual counter
    pub tick_length: chrono::Duration,                    // duration in worldtime between two ticks
    pub tick_dur: chrono::Duration,                       // duration in realtime
    pub worldtime: chrono::DateTime<chrono::FixedOffset>, // worldtime in date format
}

impl Worldtime {
    // TODO repaire tich_dur as i64 -> values is used in main and configuration
    pub fn new(tick_dur: u64, tick_length: i64) -> Self {
        Worldtime {
            tick_counter: 1,
            tick_dur: chrono::Duration::seconds(tick_dur as i64),
            tick_length: chrono::Duration::hours(tick_length),
            worldtime: chrono::DateTime::parse_from_rfc2822("1 Jan 2023 00:00:00 +0000").unwrap(),
        }
    }

    pub fn step(&mut self) {
        println!("this is a step of worldtime {}", self.worldtime);
        self.tick_counter += 1;
        thread::sleep(self.tick_dur.to_std().unwrap());
        self.worldtime = self.worldtime + self.tick_length;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_resource_worldtime() {
        let mut world = specs::World::new();
        world.add_resource(Worldtime::new(6, 2));
    }
}
