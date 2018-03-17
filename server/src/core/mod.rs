// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
use specs;
use common::configuration;

mod component;
mod system;
mod resource;
mod entity;
mod common;

pub struct Core<'a, 'b> {
    world: specs::World,
    dispatcher: specs::Dispatcher<'a, 'b>,
    playerindex: common::plystrindex::Playerstructindex,
}

impl<'a, 'b> Core<'a, 'b> {
    pub fn new(myconfig: &configuration::Configuration) -> Core<'a, 'b> {
        // Initialize
        let mut playerindex = common::plystrindex::Playerstructindex::new();

        // create the world
        let mut world = specs::World::new();

        // register components in the mod of the component sub directory
        component::new(&mut world);

        // register resources in the mod of the resource sub directory
        resource::new(&mut world, &myconfig);

        // register dispatcher
        let dispatcher = specs::DispatcherBuilder::new().build();

        // register systems in the mod of the system sub directory

        // import data
        let player1: specs::Index = entity::player::new(&mut world, "Luke".to_string());
        let player2: specs::Index = entity::player::new(&mut world, "Yoda".to_string());
        let station1: specs::Index = entity::station::new(&mut world, "ISS".to_string(), player1);
        let station2: specs::Index =
            entity::station::new(&mut world, "Moon Base".to_string(), player1);

        playerindex.add_station(player1, station1);
        println!("stationmap {:?}", playerindex);
        playerindex.add_station(player2, station2);
        println!("stationmap {:?}", playerindex);

        // return Core structure
        Core {
            world: world,
            dispatcher: dispatcher,
            playerindex: playerindex,
        }
    }

    pub fn step(self: &mut Self) {
        // update time
        {
            let mut worldtime = self.world
                .write_resource::<resource::worldtime::Worldtime>();
            worldtime.step();
        }
        // run all the systems
        self.dispatcher.dispatch(&mut self.world.res);
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
