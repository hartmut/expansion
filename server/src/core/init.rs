// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use specs;
use core::entity;
use core::component::desc::*;
use core::common::*;

pub fn init(mut world: &mut specs::World, playerindex: &mut plystrindex::Playerstructindex) {
    //currently manual insert of testdata
    // TODO import Data automagically
    let player1: specs::Index = entity::player::new(&mut world, "Luke".to_string());
    let player2: specs::Index = entity::player::new(&mut world, "Yoda".to_string());
    let station1: specs::Index = entity::station::new(&mut world, "ISS".to_string(), player1);
    let station2: specs::Index = entity::station::new(&mut world, "Moon Base".to_string(), player1);

    playerindex.add_station(player1, station1);
    println!("stationmap {:?}", playerindex);
    playerindex.add_station(player2, station2);
    println!("stationmap {:?}", playerindex);

    // COMBAK example for getting values of components
    // let name = world.read::<Desc>();
    // let catch = world.entities();
    // let catch1 = catch.fetch();
    // let entity = catch.entity(station1);
    // println!(
    //     "The name of the station is {:?}",
    //     name.get(entity).unwrap().name
    // );
}
