// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use amethyst::ecs::Entity;

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

#[cfg(test)]
mod tests {
    // use super::*;
    // use specs::prelude::*;

    // TODO new test

    // #[test]
    // fn create_player_component() {
    //     let mut world = specs::World::new();
    //     world.register::<Owner>();
    //     world.create_entity().with(Owner::new(1)).build();
    // }
}
