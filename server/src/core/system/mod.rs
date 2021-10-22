// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

mod debug_system;
pub mod update_worldtime;

use amethyst::prelude::*;

// TODO implement into bevy and create a plugin

pub fn new() -> DispatcherBuilder {
    // build Dispatcher
    let mut dispatcher = DispatcherBuilder::default();
    dispatcher
        .add_system(update_worldtime::UpdateWorldtime)
        .add_system(debug_system::DebugSystem)
        .flush();
    dispatcher
}
