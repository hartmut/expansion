// experimental initializations
use crate::core::common::formulars::*;
use crate::core::component::*;
use crate::core::entity::*;
// use crate::core::system::*;
use crate::core::common::appstate::*;
use bevy::prelude::*;
use moonshine_save::prelude::*;

pub struct InitSystem;

pub fn init(mut commands: Commands) {
    info!("Starting initialization of test data");
    // Testdata
    // create player
    // TODO create subfunctions with commands as in- and output Testit
    let player = player::Player::create(
        "Joan Piper",
        "Capitain of the first station formerly known as 'this should just work'",
    );
    let player_id = commands.spawn(player).id();
    let _player = commands.get_entity(player_id).unwrap();
    // TODO owner structure for player and NPCs, can't use bevy children as transforms would fail

    // create station record
    let station = station::Station::create("Alpha");
    let station_id = commands.spawn(station).id();

    // push children station to parent player
    // TODO use a user identity and not children/parent
    // from bevy as I don't want to project the user into the world / as resource?
    // commands.entity(player_id).push_children(&[station_id]);

    // create module and insert into last station in test
    let module = module::Module::create("Central Hub");
    let outer_volume = module.get_outer_volume();
    let first_module = commands.spawn(module).id();
    // #TODO rewrite habitat as bundle
    // commands.entity(station_id).with_child(&[first_module]);

    // add part to module
    let hab_vec = Vec3::new(3.0, 2.0, 2.0);
    if outer_volume > volume(hab_vec) {
        habitat::Habitat::add_part_habitat(commands, first_module, hab_vec, 0.0);
    }
}

// #TODO rewrite 3dassets
fn test_3dassets(mut commands: Commands, asset_server: Res<AssetServer>) {
    // IDEA integrate 3d
    // note that we have to include the `Scene0` label
    let basic: Handle<Scene> = asset_server.load("modules/basic2.gltf#Scene0");

    // spawn basic module
    commands.spawn(SceneRoot {
        ..Default::default()
    });

    // light

    // camera 
}

impl Plugin for InitSystem {
    fn build(&self, app: &mut App) {
        // insert systems for initialization in dev
        let load_save = true;
        if load_save {
            app.add_systems(Startup, load(static_file("assets/saves/world.ron")));
        } else {
            app.add_systems(Startup, init);
        }
        app.init_state::<GameState>()
            .enable_state_scoped_entities::<GameState>();

        // TODO implement loading of world
        // app.add_startup_system(load_scene::load_scene_system);

        // load assets
        // app.add_startup_system(test_3dassets);

        // register components for automatic save and inspection
        app.add_plugins((SavePlugin, LoadPlugin));
        app.register_type::<desc::Desc>();
        app.register_type::<character::Character>();
        app.register_type::<basics::BasicParameter>();
        app.register_type::<energy::Energy>();
        app.register_type::<habitat::Habitat>();
        app.register_type::<storage::Resource>();
        app.register_type::<shadow::Shadow>();
        app.register_type::<tags::ModuleTag>();
        app.register_type::<tags::StationTag>();
        app.register_type::<tags::PlayerTag>();
    }
}
