// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

extern crate bevy;
extern crate expansion;

use bevy::{log::LogPlugin, prelude::*};

use expansion::core::resource::ExpResources;

fn main() {
    let mut app = App::build();
    app.add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_plugin(ExpResources)
        .add_system_to_stage(
            CoreStage::Update,
            expansion::core::system::update_worldtime::update_worldtime.system(),
        ); // TODO configure delay between steps
    app.run();
}
