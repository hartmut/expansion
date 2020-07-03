// Copyright (C) 2016  Hartmut Prochaska
// Experimental Simulator of a cooperative solar system economy.
// See doc/LICENSE for licensing information
use core::component::*;
use specs;
use specs::prelude::*;

pub fn new(world: &mut specs::World, name: String, owner: Entity) -> Entity {
    world
        .create_entity()
        .with(PartOf::new(owner))
        .with(Desc::new(name, "".to_string()))
        .build()
}

#[cfg(test)]
mod tests {
    use super::super::super::component::partof::*;
    use super::*;
    use crate::core::entity;
    use specs;
    use specs_hierarchy::Parent as HParent;

    fn newworld() -> specs::World {
        let mut world = specs::World::new();
        super::super::super::component::new(&mut world);
        world
    }

    #[test]
    fn compare_station_ids() {
        let mut world = newworld();
        let player: Entity = entity::player::new(&mut world, "Luke".to_string());
        let station1: Entity = new(&mut world, "ISS".to_string(), player);
        let station2: Entity = new(&mut world, "Moon".to_string(), player);
        assert_ne!(station1, station2);
    }

    #[test]
    fn compare_station_entities() {
        let mut world = newworld();
        let player = super::super::player::new(&mut world, "Daniel Suarez".to_string());
        let station1 = new(&mut world, "ISS".to_string(), player);
        let station2 = new(&mut world, "Moon".to_string(), player);
        assert_ne!(station1, station2);
        let parent_storage = world.read_storage::<PartOf>();
        let parent = parent_storage.get(station1).unwrap().parent_entity();
        assert_eq!(parent, player);
    }
}
