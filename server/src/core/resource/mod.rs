// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use amethyst::{
    core::ecs::SystemBundle,
    ecs::{DispatcherBuilder, Resources, World},
    error::Error,
};

pub mod elements;
pub mod worldtime;
pub mod config;

use self::elements::ElementList;
use self::worldtime::Worldtime;
use self::config::Config;

pub struct ExpResources;

impl SystemBundle for ExpResources {
    fn load(
        &mut self,
        _world: &mut World,
        resources: &mut Resources,
        _builder: &mut DispatcherBuilder,
    ) -> Result<(), Error> {
        // read config and insert resource``
        let config = Config::load_config("resources/config.toml");

        // insert resource worldtime
        // COMBAK use parameter from config
        let worldtime = Worldtime::new(config.get_tick_length());
        resources.insert(worldtime);

        // insert resource elementlist
        // COMBAK read elementlist from file in config
        let elements = ElementList::new();
        resources.insert(elements);

        // insert resource config
        resources.insert(config);

        Ok(())
    }
}
