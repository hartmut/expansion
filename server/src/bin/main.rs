// Experimental Simulator of a cooperative solar system economy.
// See doc/LICENSE for licensing information

extern crate bevy;
extern crate expansion;

use bevy::prelude::*;

use expansion::core::resource::ExpResources;
use expansion::core::system::ExpSystems;
use expansion::init::InitSystem;
use expansion::client::Client;
use expansion::debug::MyDebug;

fn main() {
    let mut app = App::new();
    // developing in client mode because data inspection is easier
    info!("Initializing the world");
    app.add_plugin(Client) // client config
        .add_plugins(DefaultPlugins)
        .add_plugin(ExpResources) // add resources
        .add_plugin(ExpSystems) // add Systems
        .add_plugin(InitSystem) // Initialization
        .add_plugin(MyDebug);

    app.run();
}
