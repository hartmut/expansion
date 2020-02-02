// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use core::entity;
use specs;

pub fn init(mut world: &mut specs::World) {
    //currently manual insert of testdata
    // TODO import Data automagically
    let player1: specs::world::Index = entity::player::new_old(&mut world, "Luke".to_string());
    let _player2: specs::world::Index = entity::player::new_old(&mut world, "Yoda".to_string());
    let _station1: specs::world::Index =
        entity::station::new_old(&mut world, "ISS".to_string(), player1);
    let _station2: specs::world::Index =
        entity::station::new_old(&mut world, "Moon Base".to_string(), player1);
}
