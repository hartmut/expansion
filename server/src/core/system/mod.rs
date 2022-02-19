pub mod update_worldtime;
pub mod continous_save;
use bevy::{core::FixedTimestep, prelude::*};

pub struct ExpSystems;

impl Plugin for ExpSystems {
    fn build(&self, app: &mut App) {
        // insert systems for step updates
        
        // one world step every second
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second(1.0))
                .with_system(update_worldtime::update_worldtime.system())
        );

        // autosave every 60 seconds
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(30.0))
                .with_system(continous_save::continous_save.exclusive_system()),
        );
    }
}
