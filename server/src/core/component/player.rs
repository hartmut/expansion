// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// descriptions for entities
use specs;

pub struct Player {
    pub id: u32,
}

impl Player {
    pub fn new(id: u32) -> Self {
        Player { id: id }
    }
}

impl specs::Component for Player {
    type Storage = specs::HashMapStorage<Player>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_player_component() {
        let mut world = specs::World::new();
        world.register::<Player>();
        world.create_entity()
            .with(Player::new(1));
    }
}
