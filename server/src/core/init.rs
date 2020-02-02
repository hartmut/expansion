// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use core::common::*;
use core::entity;
use specs;

pub fn init(mut world: &mut specs::World, playerindex: &mut plystrindex::Playerstructindex) {
    //currently manual insert of testdata
    // TODO import Data automagically
    let player1: specs::world::Index = entity::player::new_old(&mut world, "Luke".to_string());
    let player2: specs::world::Index = entity::player::new_old(&mut world, "Yoda".to_string());
    let station1: specs::world::Index =
        entity::station::new_old(&mut world, "ISS".to_string(), player1);
    let station2: specs::world::Index =
        entity::station::new_old(&mut world, "Moon Base".to_string(), player1);

    playerindex.add_station(player1, station1);
    println!("stationmap {:?}", playerindex);
    playerindex.add_station(player2, station2);
    println!("stationmap {:?}", playerindex);

    // COMBAK example for getting values of components
    // {
    //     use core::component::desc::*;
    //     // type SystemData = specs::ReadStorage<'a, Desc>;
    //     let name = world.read::<Desc>();
    //     let catch = world.entities();
    //     let entity = catch.entity(station1);
    //     println!(
    //         "The name of the station is {:?}",
    //         name.get(catch).unwrap().name
    //     );
    // }
}
