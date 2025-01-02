use bevy::prelude::*;
use bevy::remote::http::RemoteHttpPlugin;
use bevy::remote::RemotePlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
// use bevy_editor_pls::prelude::*;
pub struct MyDebug;

impl Plugin for MyDebug {
    fn build(&self, app: &mut App) {
        // either this
        app.add_plugins(WorldInspectorPlugin::new())
            .add_plugins(RemotePlugin::default())
            .add_plugins(RemoteHttpPlugin::default());

        // or this
        // app.add_plugins(EditorPlugin::new());
    }
}
