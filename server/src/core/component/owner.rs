// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use specs;

// Owner of entities
//TODO obsolet when migrated to specs
#[derive(Debug)]
pub struct Owner {
    pub id: specs::world::Index,
}

impl Owner {
    pub fn new(id: specs::world::Index) -> Self {
        Owner { id }
    }
}

impl specs::Component for Owner {
    type Storage = specs::VecStorage<Owner>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use specs::prelude::*;

    #[test]
    fn create_player_component() {
        let mut world = specs::World::new();
        world.register::<Owner>();
        world.create_entity().with(Owner::new(1)).build();
    }
}
