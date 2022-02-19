// Experimental Simulator of a cooperative solar system economy.
use crate::core::component::account::Account;
use crate::core::component::desc::Desc;
use bevy::prelude::*;

#[derive(Bundle, Reflect)]
pub struct Player {
    desc: Desc,
    account: Account,
}

/// a player has the following parts
/// - a description,
/// - an account

impl Player {
    pub fn create(name: impl Into<String>, longtext: impl Into<String>) -> Player {
        let account = Account::new(1000);
        let desc = Desc::new(name, longtext);
        Player { desc, account }
    }
}
