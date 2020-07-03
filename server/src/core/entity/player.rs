// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
use core::component::*;
use specs;
use specs::prelude::*;

pub fn new_old(world: &mut specs::World, name: String) -> specs::world::Index {
    let player: specs::Entity = world
        .create_entity()
        .with(Account::new(1000))
        .with(Desc::new(name, "".to_string()))
        .build();
    let mut addowner = world.write_storage::<Owner>();
    let id: specs::world::Index = player.id();
    addowner.insert(player, Owner::new(id)).unwrap();
    id
}

pub fn new(world: &mut specs::World, name: String) -> Entity {
    world
        .create_entity()
        .with(Account::new(1000))
        .with(Desc::new(name, "".to_string()))
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn newworld() -> specs::World {
        let mut world = specs::World::new();
        super::super::super::component::new(&mut world);
        world
    }

    #[test]
    fn compare_player_ids() {
        let mut world = newworld();
        let player1: specs::world::Index = new_old(&mut world, "Daniel Suarez".to_string());
        let player2: specs::world::Index = new_old(&mut world, "Yoda".to_string());
        assert_ne!(player1, player2);
    }

    #[test]
    fn compare_owner_with_player_id() {
        let mut world = newworld();
        let player: Entity = new(&mut world, "Yoda".to_string());
        let _station = super::super::station::new(&mut world, "ISS".to_string(), player);
        // let player_check = station.parent.parent_entity();
        assert_eq!(player, player);
    }
}
