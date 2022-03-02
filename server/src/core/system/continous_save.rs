use crate::core::common::fileoperations::*;
use crate::core::resource::{config::*, worldtime::*};
use bevy::prelude::*;
use bevy::reflect::TypeRegistry;
use ron::ser::PrettyConfig;

pub struct ContinousSave;

pub fn continous_save(world: &mut World) {
    let type_registry = world.get_resource::<TypeRegistry>().unwrap();
    let scene = DynamicScene::from_world(world, type_registry);

    // save elements and components
    let output = scene.serialize_ron(type_registry).unwrap();
    let _f = write_string_to_file("assets/saves/quicksave_objects.ron".to_string(), &output);

    // save relevant resources
    let pretty = PrettyConfig::new()
        .separate_tuple_members(true)
        .enumerate_arrays(true);
    world
        .get_resource::<Config>()
        .unwrap()
        .save_config(pretty.clone());
    world
        .get_resource::<Worldtime>()
        .unwrap()
        .save_config(pretty.clone());

    // log
    let time = world.get_resource::<Worldtime>().unwrap().worldtime;
    info!("world save completed at world time {time}")
}
