use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

pub struct MyDebug;

impl Plugin for MyDebug {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new());
    }
}
