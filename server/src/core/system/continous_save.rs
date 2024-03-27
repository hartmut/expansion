use crate::core::resource::{config::*, worldtime::*};
use bevy::prelude::*;
use ron::ser::PrettyConfig;

pub fn continous_save_resources(world: &mut World) {
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
