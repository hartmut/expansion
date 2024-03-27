// Experimental Simulator of a cooperative solar system economy.
// See doc/LICENSE for licensing information
//
// descriptions for entities
// https://bevy-cheatbook.github.io/fundamentals/time.html
use self::super::config::Config;
use crate::core::common::fileoperations::*;
use bevy::{prelude::*, utils::*};
use hifitime::Epoch;
use ron::ser::{to_writer_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};


// Descriptions
#[derive(Resource, Debug, Serialize, Deserialize, Reflect)]
#[reflect(Resource)]
pub struct Worldtime {
    #[reflect(ignore)]
    pub epoch: Epoch, // Epoch from hifitime for orbital calculations
    pub tick_counter: u64,        // counter of current step
    pub lasttime: Duration,       // time since last call of worldtime update
    pub warp: u32,                // speedup of world- vs. real-time
    pub step_leng_warp: Duration, // duration between two steps in worldtime
}

impl Worldtime {
    pub fn default() -> Worldtime {
        Worldtime {
            epoch: Epoch::from_gregorian_utc(2030, 1, 1, 0, 0, 0, 0),
            tick_counter: 1,
            lasttime: Duration::new(0, 0),
            warp: 3600,
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

    pub fn load_config(file: String, tick_length: u32) -> Worldtime {
        // TODO replace with better loader and error handling
        let ronconfig = read_file_to_string(file);
        let mut worldtime = Worldtime::default();
        if !ronconfig.is_empty() {
            worldtime = ron::de::from_str(&ronconfig).unwrap();
        }
        worldtime.warp = tick_length * 3600;
        worldtime.lasttime = Duration::new(0, 0);
        worldtime
    }
}

impl FromWorld for Worldtime {
    fn from_world(world: &mut World) -> Self {
        let config = world.get_resource::<Config>().unwrap();
        info!("init worldtime");

        // default init worldtime
        let mut worldtime = Worldtime::default();

        let init_from_file = true;
        if init_from_file {
            // initialize from file
            worldtime = Worldtime::load_config(
                "assets/saves/resources/worldtime.ron".to_string(),
                config.get_tick_length(),
            );
        }
        // reset timer
        worldtime.step_leng_warp = Duration::new(0, 0);
        worldtime
    }
}
