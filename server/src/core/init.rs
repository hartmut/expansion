// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use core::entity;
use core::entity::station;
use specs;
use specs::prelude::*;

pub fn init(mut world: &mut specs::World) {
    //currently manual insert of testdata
    // TODO import resources automagically
    let player1: Entity = entity::player::new(&mut world, "Luke".to_string());
    let _player2: Entity = entity::player::new(&mut world, "Yoda".to_string());
    let player3: Entity = entity::player::new(&mut world, "FitzRoy".to_string());
    let _station1: Entity = station::new(&mut world, "ISS".to_string(), player1);
    let _station2: Entity = station::new(&mut world, "Moon Base".to_string(), player1);
    let _station3: Entity = station::new(&mut world, "Beagle".to_string(), player3);
}
