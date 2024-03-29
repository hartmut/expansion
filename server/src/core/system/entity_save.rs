// use crate::core::common::fileoperations::*;
use crate::core::resource::worldtime::*;
use bevy::prelude::*;
// use bevy::reflect::TypeRegistry;
// use crate::core::entity::*;


// export all entities as respectively into theire own files
pub fn entity_save (world: &mut World) {
    // TODO rewrite entity_save if necessary
    // let type_registry = world.get_resource::<TypeRegistry>().unwrap();
    // let scene = DynamicScene::from_world(world, type_registry);
    // let output = scene.serialize_ron(type_registry).unwrap();
    // let _f = write_string_to_file("assets/saves/entities/entity.scn.ron".to_string(), &output);

    // log
    let time = world.get_resource::<Worldtime>().unwrap().epoch;
    info!("entity save {time}")
}
