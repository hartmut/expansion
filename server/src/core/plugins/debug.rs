use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_editor_pls::prelude::*;
pub struct MyDebug;

impl Plugin for MyDebug {
    fn build(&self, app: &mut App) {
        // either this
        // app.add_plugin(WorldInspectorPlugin::new());

        // or this
        app.add_plugin(EditorPlugin);
    }
}
