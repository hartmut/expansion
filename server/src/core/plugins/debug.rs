use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
// use bevy_editor_pls::prelude::*;
pub struct MyDebug;

impl Plugin for MyDebug {
    fn build(&self, app: &mut App) {
        // either this
        app.add_plugins(WorldInspectorPlugin::new());

        // or this
        // app.add_plugins(EditorPlugin::new());
    }
}
