// Experimental Simulator of a cooperative solar system economy.
// See doc/LICENSE for licensing information

extern crate bevy;
extern crate expansion;

use bevy::prelude::*;

use expansion::core::plugins::*;
use expansion::core::resource::ExpResources;
use expansion::core::system::ExpSystems;
use bevy_editor_pls::prelude::*;

fn main() {
    info!("Initializing the world");
    App::new()
        .add_plugins(init::InitSystem) // Initialization
        .add_plugins(ExpResources) // add resources
        .add_plugins(ExpSystems) // add Systems
        // COMEBACK UI
        .add_plugins(ui::Ui) // client config
        .add_plugins(debug::MyDebug)
        .add_plugins(EditorPlugin::default())
        .run();
}
