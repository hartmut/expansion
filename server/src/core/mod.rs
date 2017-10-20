// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
use specs;
use common::configuration;
use std::time::Duration;

mod component;
mod system;
mod resource;
mod entity;

pub struct Core<'a, 'b> {
    world: specs::World,
    dispatcher: specs::Dispatcher<'a, 'b>,
}

impl<'a, 'b> Core<'a, 'b> {
    pub fn new(myconfig: configuration::Configuration) -> Core<'a, 'b> {

        // create world and register components
        let mut world = specs::World::new();

        // register components
        component::new(&mut world);

        // register resources
        resource::new(&mut world, &myconfig);

        // register dispatcher
        let dispatcher = specs::DispatcherBuilder::new().build();

        // register systems

        // import data
        let _player: specs::Index = entity::player::new(&mut world, "Daniel Suarez".to_string());
        let _player: specs::Index = entity::player::new(&mut world, "Yoda".to_string());

        // return Core structure
        Core {
            world: world,
            dispatcher: dispatcher,
        }
    }

    pub fn step(self: &mut Self) {
        // println!("one step", );
        self.dispatcher.dispatch(&mut self.world.res);
    }
}
