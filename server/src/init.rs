// experimental initializations
use crate::core::component::*;
use crate::core::entity::*;
use bevy::prelude::*;
use bevy_inspector_egui::InspectableRegistry;

pub fn init(mut commands: Commands) {
    info!("Starting initialization");
    // Testdata
    // create player
    let player = player::Player::create("Riker", "First Officer");
    let player_id = commands.spawn_bundle(player).id();
    // create station record
    let station = station::Station::create("ISS");
    let station_id = commands.spawn_bundle(station).id();
    // push children station to parent player
    commands.entity(player_id).push_children(&[station_id]);
    // create module and insert into last station in test
    let module = module::Module::create("Central Hub");
    let first_module = commands.spawn_bundle(module).id();
    commands.entity(station_id).push_children(&[first_module]);
    // TODO import entities
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
        registry.register::<energy::Energy>();
        registry.register::<habitat::Habitat>();
        registry.register::<resources::Resource>();
    }
}
