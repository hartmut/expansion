// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use amethyst::{
    core::ecs::SystemBundle,
    ecs::{DispatcherBuilder, Resources, World},
    error::Error,
};

pub mod config;
pub mod elements;
pub mod worldtime;

use self::config::Config;
use self::elements::ElementList;
use self::worldtime::Worldtime;
use bevy::prelude::*;

pub struct ExpResources;

impl SystemBundle for ExpResources {
    fn load(
        &mut self,
        _world: &mut World,
        resources: &mut Resources,
        _builder: &mut DispatcherBuilder,
    ) -> Result<(), Error> {
        // read config and insert resource
        let config = Config::load_config("resources/config.toml");

        // insert resource worldtime
        let worldtime = Worldtime::new(config.get_tick_length());
        resources.insert(worldtime);

        // insert resource elementlist
        let elements = ElementList::init();
        resources.insert(elements);

        // insert resource config
        resources.insert(config);

        Ok(())
    }
}

impl Plugin for ExpResources {
    fn build(&self, app: &mut AppBuilder) {
        // TODO use data source for elementlist from configfile

        // insert resources
        app.init_resource::<Config>()
            .init_resource::<Worldtime>()
            .init_resource::<ElementList>();
    }
}
