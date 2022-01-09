pub mod update_worldtime;
use bevy::{core::FixedTimestep, prelude::*};

pub struct ExpSystems;

impl Plugin for ExpSystems {
    fn build(&self, app: &mut App) {
        // insert systems for step updates
        
        // updates 30 times a second, for physics and graphics
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second(1.0))
                .with_system(update_worldtime::update_worldtime.system()),
        );
        // TODO insert system for automatic world save every x second(s) in the background - performance
    }
}
