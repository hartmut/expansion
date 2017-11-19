// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use specs;
use common::configuration;

pub mod worldtime;
pub use self::worldtime::Worldtime;

pub fn new(world: &mut specs::World, myconfig: &configuration::Configuration) {
    world.add_resource(worldtime::Worldtime::new(myconfig.get_tick(), myconfig.get_tick_length()));
}
