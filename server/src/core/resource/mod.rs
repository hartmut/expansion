// Experimental Simulator of a cooperative solar system economy.
// See doc/LICENSE for licensing information

pub mod config;
pub mod elements;
pub mod worldtime;

use self::config::Config;
use self::elements::ElementList;
use self::worldtime::Worldtime;
use bevy::prelude::*;

pub struct ExpResources;

impl Plugin for ExpResources {
    fn build(&self, app: &mut App) {
        // TODO use data source for elementlist from configfile

        // insert resources
        app.init_resource::<Config>()
            .init_resource::<Worldtime>()
            .init_resource::<ElementList>();
    }
}
