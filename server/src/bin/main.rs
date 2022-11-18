// Experimental Simulator of a cooperative solar system economy.
// See doc/LICENSE for licensing information

extern crate bevy;
extern crate expansion;

use bevy::prelude::*;

use expansion::core::resource::ExpResources;
use expansion::core::system::ExpSystems;
use expansion::core::plugins::*;

fn main() {
    let mut app = App::new();
    // developing in client mode because data inspection is easier
    info!("Initializing the world");
    app.add_plugin(defaultsettings::Defaultsettings) // client config
        // .add_plugins(DefaultPlugins) // already inserted in Client Plugin
        .add_plugin(ExpResources) // add resources
        .add_plugin(ExpSystems) // add Systems
        .add_plugin(init::InitSystem) // Initialization
        .add_plugin(debug::MyDebug);

    app.run();
}
