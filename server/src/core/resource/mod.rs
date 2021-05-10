// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use amethyst::{
    core::ecs::SystemBundle,
    ecs::{DispatcherBuilder, Resources, World},
    error::Error,
};

pub mod worldtime;
pub use self::worldtime::Worldtime;

pub struct ExpResources;

impl SystemBundle for ExpResources {
    fn load(
        &mut self,
        _world: &mut World,
        resources: &mut Resources,
        _builder: &mut DispatcherBuilder,
    ) -> Result<(), Error> {
        let worldtime = Worldtime::new(6);
        resources.insert(worldtime);

        Ok(())
    }
}
