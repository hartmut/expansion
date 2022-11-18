// use crate::core::common::fileoperations::*;
use crate::core::resource::worldtime::*;
use bevy::prelude::*;
// use bevy::reflect::TypeRegistry;
// use crate::core::entity::*;


// export all entities as respectively into theire own files
pub fn entity_save (world: &mut World) {
    // let type_registry = world.get_resource::<TypeRegistry>().unwrap();
    // let mut helperworld: World = FromWorld::from_world(world);
    // let station = station::Station::create("Alpha");
    // helperworld.spawn_bundle(station);
    
    // COMEBACK 
    // let scene = DynamicScene::from_world(world, type_registry);
    // let output = scene.serialize_ron(type_registry).unwrap();
    // let _f = write_string_to_file("assets/saves/entities/entity.scn.ron".to_string(), &output);

    // log
    let time = world.get_resource::<Worldtime>().unwrap().worldtime;
    info!("entity save {time}")
}
