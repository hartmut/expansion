// Copyright (C) 2016  Hartmut Prochaska
// Experimental Simulator of a cooperative solar system economy.
// See doc/LICENSE for licensing information
use amethyst::prelude::*;
use core::component::desc::Desc;

pub struct Station;

impl Station {
    pub fn new(world: &mut World, name: String, _owner: Entity) -> Entity {
        let desc = Desc::new(name, "".to_string());
        world.push((desc,))
    }
}
