// Experimental Simulator of a cooperative solar system economy.
// See doc/LICENSE for licensing information

extern crate bevy;
extern crate expansion;

use bevy::prelude::*;

use expansion::core::plugins::*;
use expansion::core::resource::ExpResources;
use expansion::core::system::ExpSystems;

fn main() {
    info!("Initializing the world");
    App::new()
        .add_plugin(init::InitSystem) // Initialization
        .add_plugin(ExpResources) // add resources
        .add_plugin(ExpSystems) // add Systems
        .add_plugin(ui::Ui) // client config
        .add_plugin(debug::MyDebug)
        .run();
}
