// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use specs;

// Part of a bigger structure
#[derive(Debug)]
pub struct Partof {
    id: specs::Index,
}

impl Partof {
    pub fn new(id: specs::Index) -> Self {
        Partof { id: id }
    }

    pub fn modify(&mut self, id: specs::Index) {
        self.id = id;
    }
}

impl specs::Component for Partof {
    type Storage = specs::VecStorage<Partof>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_partof_component() {
        let mut world = specs::World::new();
        world.register::<Partof>();
        world.create_entity().with(Partof::new(1));
    }

    #[test]
    fn modify_partof_component() {
        let mut part = Partof::new(1);
        part.modify(2);
        assert_eq!(part.id, 2);
    }
}
