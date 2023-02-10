use bevy::prelude::*;
pub struct Ui;

impl Plugin for Ui {
    fn build(&self, app: &mut App) {
        let height: f32 = 700.0;

        // create window
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                height,
                width: height * (16.0 / 9.0),
                title: "Expansion".to_string(),
                resizable: true,
                ..default()
            },
            ..default()
        }));
    }
}
