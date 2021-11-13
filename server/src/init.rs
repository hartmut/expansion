// experimental initializations
use crate::core::component::*;
use crate::core::entity::*;
use bevy::prelude::*;
use bevy::transform::prelude::Parent;
use bevy_inspector_egui::InspectableRegistry;

pub fn init(mut commands: Commands) {
    info!("Starting initialization");
    // Testdata
    let player_id = player::Player::create(commands.spawn(), "Riker", "First Officer");
    let station_id = station::Station::create(commands.spawn(), "ISS");
    commands.entity(player_id).push_children(&[station_id]);
    let first_module = module::Module::create(commands.spawn(), "Central Hub");
    commands.entity(station_id).push_children(&[first_module]);
    // COMEBACK import entities
}

pub struct InitSystem;

impl Plugin for InitSystem {
    fn build(&self, app: &mut AppBuilder) {
        // insert systems for initialization in dev
        app.add_startup_system(init.system());

        let mut registry = app
            .world_mut()
            .get_resource_mut::<InspectableRegistry>()
            .unwrap();
        registry.register::<desc::Desc>();
        registry.register::<account::Account>();
        registry.register::<basics::BasicParameter>();
    }
}
