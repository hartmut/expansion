// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
use amethyst::prelude::*;
use core::component::account::Account;
use core::component::desc::Desc;

pub struct Player;

impl Player {
    pub fn new(world: &mut World, name: String) -> Entity {
        let account = Account::new(1000);
        let desc = Desc::new(name, "".to_string());
        world.push((account, desc))
    }
}
