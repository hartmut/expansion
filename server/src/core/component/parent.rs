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
    use specs_hierarchy::{Hierarchy, HierarchySystem};

    fn newworld() -> specs::World {
        let mut world = specs::World::new();
        super::super::super::component::new(&mut world);
        world
    }

    #[test]
    fn create_player_component() {
        let mut world = newworld();
        let id = world
            .create_entity()
            .with(desc::Desc::new("Daniel Suarez".to_string(), "".to_string()))
            .build();
        world.create_entity().with(Parent::new(id)).build();
    }

    #[test]
    fn test_hierarchy() {
        // init the world
        let mut world = newworld();
        let mut dispatcher = specs::DispatcherBuilder::new()
            .with(
                HierarchySystem::<Parent>::new(&mut world),
                "hierarchy_system",
                &[],
            )
            .build();
        dispatcher.setup(&mut world);

        // create test Hierarchy
        let parent = world
            .create_entity()
            .with(desc::Desc::new("Daniel Suarez".to_string(), "".to_string()))
            .build();
        let child = world.create_entity().with(Parent::new(parent)).build();

        // one step
        dispatcher.dispatch(&mut world);

        // exexute test
        let resource = world.fetch::<Hierarchy<Parent>>();
        let childtest = resource.children(parent);
        assert_eq!(child, childtest[0]);
    }
}
