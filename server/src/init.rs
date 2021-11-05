// experimental initializations
use crate::core::entity::*;
use bevy::prelude::*;

// TODO create initial components
pub fn init(commands: Commands) {
    info!("Starting initialization");
    let _player_id = player::Player::create(commands, "Riker", "First Officer");
}

pub struct InitSystem;

impl Plugin for InitSystem {
    fn build(&self, app: &mut AppBuilder) {
        // insert systems for initialization in dev
        app.add_startup_system(init.system());
    }
}
