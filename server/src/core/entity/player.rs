// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
use specs;
use core::component::*;

pub fn new(world: &mut specs::World, name: String) -> specs::Index {
    let player: specs::Entity = world.create_entity()
        .with(Account { credits: 1000 })
        .with(Desc {
            name: name,
            longname: "".to_string(),
        })
        .build();
    player.id()
}
