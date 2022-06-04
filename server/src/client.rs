use bevy::prelude::*;

pub struct Client;
impl Plugin for Client {
    fn build(&self, app: &mut App) {
        let height: f32 = 700.0;
        app.insert_resource(WindowDescriptor {
            height,
            width: height * (16.0 / 9.0),
            title: "Expansion".to_string(),
            resizable: true,
            ..Default::default()
        });
    }
}
