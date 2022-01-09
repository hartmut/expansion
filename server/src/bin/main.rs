// Experimental Simulator of a cooperative solar system economy.
// See doc/LICENSE for licensing information

extern crate bevy;
extern crate expansion;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

use expansion::core::resource::ExpResources;
use expansion::core::system::ExpSystems;
use expansion::init::InitSystem;

fn main() {
    let mut app = App::new();
    // developing in client mode because data inspection is easier
    info!("Initializing the world");
    app.add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new()) 
        .add_plugin(ExpResources) // add resources
        .add_plugin(ExpSystems) // add Systems
        .add_plugin(InitSystem); // Initialization

    app.run();
}
