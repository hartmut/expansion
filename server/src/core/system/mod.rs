// mod debug_system;
pub mod update_worldtime;
use bevy::{core::FixedTimestep, prelude::*};

pub struct ExpSystems;

impl Plugin for ExpSystems {
    fn build(&self, app: &mut AppBuilder) {
        // insert systems for step updates
        
        // updates 30 times a second, for physics and graphics
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second(30.0))
                .with_system(update_worldtime::update_worldtime.system()),
        );
    }
}
