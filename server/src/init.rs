// experimental initializations
use crate::core::component::*;
use crate::core::entity::*;
use crate::core::system::*;
use bevy::prelude::*;
// use bevy_inspector_egui::InspectableRegistry;

pub fn init(mut commands: Commands) {
    info!("Starting initialization");
    // Testdata
    // create player
    let player = player::Player::create("Picard", "Capitain");
    let player_id = commands.spawn_bundle(player).id();
    // create station record
    let station = station::Station::create("Enterprise");
    let station_id = commands.spawn_bundle(station).id();
    // push children station to parent player
    commands.entity(player_id).push_children(&[station_id]);
    // create module and insert into last station in test
    let module = module::Module::create("Central Hub");
    let first_module = commands.spawn_bundle(module).id();
    commands.entity(station_id).push_children(&[first_module]);
    // TODO implement automatic load of the last quicksave or other scene saves
}

pub struct InitSystem;

impl Plugin for InitSystem {
    fn build(&self, app: &mut App) {
        // insert systems for initialization in dev
        // app.add_startup_system(init);
        app.add_startup_system(load_scene::load_scene_system);

        // register components for automatic save
        app.register_type::<desc::Desc>();
        app.register_type::<account::Account>();
        app.register_type::<basics::BasicParameter>();
        app.register_type::<energy::Energy>();
        app.register_type::<habitat::Habitat>();
        app.register_type::<resources::Resource>();
        app.register_type::<shadow::Shadow>();
    }
}
