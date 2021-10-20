// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use bevy::ecs::entity::Entity;

// TODO implement Owner/Parent structure for stations and player as owner, propably bevy has this out of the box
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
