pub mod continous_save;
pub mod load_scene;
pub mod shadow_systems;
pub mod update_worldtime;
use bevy::{core::FixedTimestep, prelude::*};

pub struct ExpSystems;

impl Plugin for ExpSystems {
    fn build(&self, app: &mut App) {
        // insert systems for step updates

        // one world step every second
        app.add_system_set(
            SystemSet::new()
                .label("OneSecond")
                .with_run_criteria(FixedTimestep::steps_per_second(1.0))
                .with_system(shadow_systems::shadow_clear.label("shadow clear"))
                .with_system(shadow_systems::shadow_update_module.after("shadow clear").label("shadow update module"))
                .with_system(shadow_systems::shadow_update_station.after("shadow update module"))
                .with_system(update_worldtime::update_worldtime),
        );

        // autosave every x seconds
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(5.0))
                .with_system(continous_save::continous_save.exclusive_system()),
        );
    }
}
