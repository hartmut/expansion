use bevy::prelude::*;
use bevy::reflect::TypeRegistry;
use crate::core::common::fileoperations::*;

pub struct ContinousSave;

pub fn continous_save(world: &mut World) {

    let type_registry =  world.get_resource::<TypeRegistry>().unwrap();
    let scene = DynamicScene::from_world(world, type_registry);

    let output = scene
            .serialize_ron(type_registry)
            .unwrap();
    let _f = write_string_to_file("assets/saves/quicksave_objects.ron".to_string(), &output);
}
