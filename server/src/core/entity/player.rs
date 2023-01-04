// Experimental Simulator of a cooperative solar system economy.
use crate::core::component::*;
use bevy::prelude::*;

#[derive(Bundle, Reflect)]
pub struct Player {
    desc: desc::Desc,
    account: character::Character,
    playertag: tags::PlayerTag,
}

/// a player has the following parts
/// - a description,
/// - an account

impl Player {
    pub fn create(name: impl Into<String>, longtext: impl Into<String>) -> Player {
        let account = character::Character::new(1000);
        let desc = desc::Desc::new(name, longtext);
        let playertag = tags::PlayerTag;
        Player { desc, account, playertag }
    }
}
