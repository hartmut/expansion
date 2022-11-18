use bevy::prelude::*;
pub struct Defaultsettings;

impl Plugin for Defaultsettings {
    fn build(&self, app: &mut App) {
        let height: f32 = 700.0;

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
