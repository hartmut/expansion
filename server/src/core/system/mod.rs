// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

pub mod update_worldtime;

use amethyst::prelude::*;

// TODO change to bundle loader
pub fn new() -> DispatcherBuilder {
    // build Dispatcher
    let mut dispatcher = DispatcherBuilder::default();
    dispatcher.add_system(update_worldtime::UpdateWorldtime);
    dispatcher
}
