use crate::core::common::fileoperations::*;
use crate::core::resource::{config::*, worldtime::*};
use bevy::prelude::*;
use ron::ser::PrettyConfig;

// TODO panic becaus of bevy_math::rect::Rect when saving in line 12 - bug in Bevy?
pub fn continous_save(world: &mut World) {
    let type_registry = world.get_resource::<AppTypeRegistry>().unwrap();
    let scene = DynamicScene::from_world(world);

    // TODO save elements and components, this doesn't work anymore, need to control more which data to save
    // check https://github.com/Zeenobit/moonshine_save
    // let output = scene.serialize_ron(type_registry).unwrap();
    // let _f = write_string_to_file("assets/saves/quicksave_objects.scn.ron".to_string(), &output);

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
        .save_config(pretty);

    // log
    let time = world.get_resource::<Worldtime>().unwrap().epoch;
    info!("world save completed at world time {time}")
}
