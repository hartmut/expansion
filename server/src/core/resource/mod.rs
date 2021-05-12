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

pub use self::elements::ElementList;
pub use self::worldtime::Worldtime;
use crate::utils::config;

pub struct ExpResources;

impl SystemBundle for ExpResources {
    fn load(
        &mut self,
        _world: &mut World,
        resources: &mut Resources,
        _builder: &mut DispatcherBuilder,
    ) -> Result<(), Error> {
        // read config and insert resource``
        let _myconfig = config::Config::load_config("resources/config.toml");

        // insert resource worldtime
        // COMBAK use parameter from config
        let worldtime = Worldtime::new(6);
        resources.insert(worldtime);

        // insert resource elementlist
        // COMBAK read elementlist from file in config
        let elements = ElementList::new();
        resources.insert(elements);

        // COMBAK add myconfig as resource

        Ok(())
    }
}
