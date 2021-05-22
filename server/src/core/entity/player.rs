// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
use amethyst::{core::Transform, prelude::*};
use core::component::account::Account;
use core::component::desc::Desc;

pub struct Player;

/// a player has the following parts
/// - a description,
/// - an account
/// - some stations as as children

impl Player {
    pub fn new(world: &mut World, name: String) -> Entity {
        let account = Account::new(1000);
        let desc = Desc::new(name, "".to_string());
        let transform = Transform::default();
        world.push((account, desc, transform))
    }
}
