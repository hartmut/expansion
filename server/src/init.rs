// experimental initializations
use crate::core::component::*;
use crate::core::entity::*;
use crate::core::system::*;
use bevy::prelude::*;

pub fn init(mut commands: Commands) {
    info!("Starting initialization of further test data");
    // Testdata
    // create player
    // TODO create subfunctions with commands as in- and output Testit
    let player = player::Player::create("Joan Piper", "Capitain of the first station formerly known as 'this should just work'");
    let player_id = commands.spawn_bundle(player).id();
    // create station record
    let station = station::Station::create("Alpha");
    let station_id = commands.spawn_bundle(station).id();
    // push children station to parent player
    commands.entity(player_id).push_children(&[station_id]);
    // create module and insert into last station in test
    let module = module::Module::create("Central Hub");
    let outer_volume = module.get_outer_volume();
    let first_module = commands.spawn_bundle(module).id();
    commands.entity(station_id).push_children(&[first_module]);
    // add component to entity
    // TODO change habitat to a part (component in a module)
    match habitat::Habitat::add_habitat(40., outer_volume) {
        Ok(habitat) => {
            commands.entity(first_module).insert(habitat);
            debug!("adding of habitat successfull")
        }
        Err(err) => info!("{}", err),
    };
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
        app.register_type::<tags::ModuleTag>();
        app.register_type::<tags::StationTag>();
    }
}
