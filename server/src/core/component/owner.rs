// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// descriptions for entities
use specs;

// Owner of entities
pub struct Owner {
    pub id: specs::Index,
}

impl Owner {
    pub fn new(id: specs::Index) -> Self {
        Owner { id: id }
    }
}

impl specs::Component for Owner {
    type Storage = specs::VecStorage<Owner>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_player_component() {
        let mut world = specs::World::new();
        world.register::<Owner>();
        world.create_entity()
            .with(Owner::new(1));
    }
}
