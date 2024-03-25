use crate::core::common::fileoperations::*;
use crate::core::resource::{config::*, worldtime::*};
use bevy::prelude::*;
use ron::ser::PrettyConfig;

// TODO panic becaus of bevy_math::rect::Rect when saving in line 12 - bug in Bevy?
pub fn continous_save(world: &mut World) {
    // https://github.com/bevyengine/bevy/discussions/12063#discussioncomment-8571129

    let type_registry = world.resource::<AppTypeRegistry>();
    let scene = DynamicScene::from_world(&world);
    // let output = scene.serialize_ron(type_registry).unwrap();

    // TODO save elements and components, this doesn't work anymore, need to control more which data to save
    // check https://github.com/Zeenobit/moonshine_save

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
    // let time = world.get_resource::<Worldtime>().unwrap().epoch;
    // info!("world save completed at world time {time}")
}
