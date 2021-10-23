// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

extern crate bevy;
extern crate expansion;

use bevy::{log::LogPlugin, prelude::*};
use bevy_inspector_egui::WorldInspectorPlugin;

use expansion::core::resource::ExpResources;
use expansion::core::system::ExpSystems;

fn main() {
    let mut app = App::build();
    app.add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ExpResources)
        .add_plugin(ExpSystems);
    app.run(); //TODO limiter on frames/sec 
}
