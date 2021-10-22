// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

extern crate amethyst;
extern crate bevy;
extern crate expansion;

use bevy::{log::LogPlugin, prelude::*};

use expansion::core::resource::ExpResources;

// my mods to use
// use expansion::core::Core;

fn main() -> amethyst::Result<()> {
    // Bevy section
    let mut app = App::build();
    app.add_plugin(LogPlugin::default())
        .add_plugin(ExpResources)
        .add_system(expansion::core::system::update_worldtime::update_worldtime.system()); //BUG runs only once, add to running stage

    app.run();

    Ok(())
}
