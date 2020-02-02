// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use specs::prelude::{Component, DenseVecStorage, Entity, FlaggedStorage};
use specs_hierarchy::Parent as HParent;

// Owner of entities
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Parent {
    pub parent: Entity,
}

impl HParent for Parent {
    fn parent_entity(&self) -> Entity {
        self.parent
    }
}

impl Parent {
    pub fn new(id: Entity) -> Parent {
        Parent { parent: id }
    }
}

impl Component for Parent {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

#[cfg(test)]
mod tests {
    use super::super::desc;
    use super::*;
    use specs::prelude::*;

    #[test]
    fn create_player_component() {
        let mut world = specs::World::new();
        world.register::<Parent>();
        world.register::<desc::Desc>();
        let id = world
            .create_entity()
            .with(desc::Desc::new("Daniel Suarez".to_string(), "".to_string()))
            .build();
        world.create_entity().with(Parent::new(id)).build();
    }
}
