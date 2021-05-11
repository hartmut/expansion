// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

// Money
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Account {
    credits: u64,
}

impl Account {
    pub fn new(credits: u64) -> Self {
        Account { credits }
    }
}


#[cfg(test)]
mod tests {
    // use super::*;
    // use specs::prelude::*;
    // use specs::world::Builder;

// TODO rewrite test
    // #[test]
    // fn create_player_component() {
    //     let mut world = specs::World::new();
    //     world.register::<Account>();
    //     world.create_entity().with(Account::new(100)).build();
    // }
}
