// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use utils::config;

pub mod worldtime;
pub use self::worldtime::Worldtime;

pub fn new(world: &mut specs::World, myconfig: &config::Config) {
    world.insert(worldtime::Worldtime::new(
        myconfig.get_tick(),
        myconfig.get_tick_length(),
    ));
}
