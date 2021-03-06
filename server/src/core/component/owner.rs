// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use amethyst::ecs::Entity;

// TODO implement Owner/Parent structure independent of amethyst for stations and player as owner
// Owner of entities
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Owner {
    pub id: Entity,
}

impl Owner {
    pub fn new(id: Entity) -> Self {
        Owner { id }
    }
}
