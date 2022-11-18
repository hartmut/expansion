// experimental initializations
use crate::core::component::*;
use crate::core::entity::*;
use crate::core::common::formulars::*;
// use crate::core::system::*;
use bevy::prelude::*;

pub fn init(mut commands: Commands) {
    info!("Starting initialization of further test data");
    // Testdata
    // create player
    // TODO create subfunctions with commands as in- and output Testit
    let player = player::Player::create(
        "Joan Piper",
        "Capitain of the first station formerly known as 'this should just work'",
    );
    let player_id = commands.spawn(player).id();
    // create station record
    let station = station::Station::create("Alpha");
    let station_id = commands.spawn(station).id();
    // push children station to parent player
    commands.entity(player_id).push_children(&[station_id]);
    // create module and insert into last station in test
    let module = module::Module::create("Central Hub");
    let outer_volume = module.get_outer_volume();
    let first_module = commands.spawn(module).id();
    commands.entity(station_id).push_children(&[first_module]);

    // add part to module
    let hab_vec = Vec3::new(3.0, 2.0, 2.0);
    if outer_volume > volume(hab_vec) {
        commands = habitat::Habitat::add_part_habitat(commands, first_module, hab_vec, 0.0);
    }
}

pub fn testinit(mut commands: Commands) {
    info!("Starting initialization of further test data");
    // Testdata
    // create player
    // create station record
    let module = module::Module::create("Central Hub");
    let _first_module = commands.spawn(module).id();
}

pub struct InitSystem;

impl Plugin for InitSystem {
    fn build(&self, app: &mut App) {
        // insert systems for initialization in dev
        app.add_startup_system(init);
        // app.add_startup_system(testinit);
        // TODO implement loading of world 
        // app.add_startup_system(load_scene::load_scene_system);

        // register components for automatic save
        app.register_type::<desc::Desc>();
        app.register_type::<account::Account>();
        app.register_type::<basics::BasicParameter>();
        app.register_type::<energy::Energy>();
        app.register_type::<habitat::Habitat>();
        app.register_type::<storage::Resource>();
        app.register_type::<shadow::Shadow>();
        app.register_type::<tags::ModuleTag>();
        app.register_type::<tags::StationTag>();
    }
}
