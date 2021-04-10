// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
use utils::configuration;
use specs::prelude::*;
pub mod component;
pub mod states;
mod common;
mod entity;
mod init;
mod resource;
mod system;

// use core;

pub struct Core<'a, 'b> {
    world: specs::World,
    dispatcher: specs::Dispatcher<'a, 'b>,
}

impl<'a, 'b> Core<'a, 'b> {
    pub fn new(myconfig: &configuration::Configuration) -> Core<'a, 'b> {
        // create the world
        let mut world = specs::World::new();

        // register components in the mod of the component sub directory
        component::new(&mut world);

        // register resources in the mod of the resource sub directory
        resource::new(&mut world, &myconfig);

        //register systems
        let dispatcher = system::new(&mut world);
        init::init(&mut world);

        // return Core structure
        Core { world, dispatcher }
    }

    pub fn step(self: &mut Self) {
        // update time
        {
            let mut worldtime = self
                .world
                .write_resource::<resource::worldtime::Worldtime>();
            worldtime.step();
        }
        // run all the systems
        self.dispatcher.dispatch(&mut self.world);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newworld_with_components() {
        let mut world = specs::World::new();
        component::new(&mut world);
    }
}
