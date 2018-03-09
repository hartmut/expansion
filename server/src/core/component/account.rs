// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use specs;

// Money
#[derive(Debug)]
pub struct Account {
    credits: u64,
}

impl Account {
    pub fn new(credits: u64) -> Self {
        Account { credits: credits }
    }
}

impl specs::Component for Account {
    type Storage = specs::HashMapStorage<Account>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_player_component() {
        let mut world = specs::World::new();
        world.register::<Account>();
        world.create_entity()
            .with(Account::new(100));
    }
}
