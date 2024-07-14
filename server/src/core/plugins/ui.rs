use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
pub struct Ui;

impl Plugin for Ui {
    fn build(&self, app: &mut App) {
        let height: f32 = 700.0;
        let resolution = bevy::window::WindowResolution::new(height * (16.0 / 9.0), height);

        // create window
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution,
                title: "Expansion".to_string(),
                resizable: true,
                ..default()
            }),
            ..default()
        }));
        // .add_plugins(EditorPlugin::default());
    }
}
