// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
use specs;
use core::component::*;

pub fn new(world: &mut specs::World, name: String) -> specs::Index {
    let player: specs::Entity = world.create_entity()
        .with(Account { credits: 1000 })
        .with(Desc {
            name: name,
            longname: "".to_string(),
        })
        .build();
    let mut addowner = world.write::<Owner>();
    let id: specs::Index = player.id();
    addowner.insert(player, Owner::new(id));
    println!("{:?}", player);
    id
}

#[cfg(test)]
mod tests {
    use super::*;

    fn newworld() -> specs::World {
        let mut world = specs::World::new();
        world.register::<Owner>();
        world.register::<Desc>();
        world.register::<Account>();
        world
    }

    #[test]
    fn compare_player_ids() {
        let mut world = newworld();
        let player1: specs::Index = new(&mut world, "Daniel Suarez".to_string());
        let player2: specs::Index = new(&mut world, "Yoda".to_string());
        assert_ne!(player1, player2);
    }

    #[test]
    fn compare_owner_with_player_id() {
        let mut world = newworld();
        let player: specs::Index = new(&mut world, "Yoda".to_string());
        let fetch: specs::Fetch<specs::EntitiesRes> = world.entities();
        let deref = fetch.deref();
        println!("{:?}", deref);
        // let readowner = world.read_with_id::<Owner>(player as usize);
        // let owner = readowner.get(player as usize);
        // TODO read the owner of this player
        assert_eq!(player, player);
    }
}
