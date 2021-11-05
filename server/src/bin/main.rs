// Experimental Simulator of a cooperative solar system economy.
// See doc/LICENSE for licensing information

extern crate bevy;
extern crate expansion;

use bevy::prelude::*;
use bevy_inspector_egui::{InspectorPlugin, WorldInspectorPlugin};

use expansion::core::component::desc::Desc;
use expansion::core::resource::ExpResources;
use expansion::core::system::ExpSystems;
use expansion::init::InitSystem;

fn main() {
    let mut app = App::build();
    // developing in client mode because data inspection is easier
    info!("Initializing the world");
    app.add_plugins(DefaultPlugins) // VERSION1 needs to change for servermode to necessary plugins without window mode
        .add_plugin(WorldInspectorPlugin::new()) // VERSION1 not needed in servermode
        .add_plugin(InspectorPlugin::<Desc>::new())
        .add_plugin(ExpResources) // add resources
        .add_plugin(ExpSystems) // add Systems
        .add_plugin(InitSystem); // Initialization
    app.run();
}
