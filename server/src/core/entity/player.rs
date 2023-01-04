use std::fmt::Display;

// Experimental Simulator of a cooperative solar system economy.
use crate::core::component::*;
use bevy::prelude::*;

#[derive(Bundle, Reflect)]
pub struct Player {
    desc: desc::Desc,
    account: character::Character,
    playertag: tags::PlayerTag,
    name: Name,
}

/// a player has the following parts
/// - a description,
/// - an account

impl Player {
    pub fn create(
        name_player: impl Into<String> + Clone + Display,
        longtext: impl Into<String>,
    ) -> Player {
        let account = character::Character::new(1000);
        let name_entity = name_player.clone().to_string();
        let name = Name::new(name_entity);
        let desc = desc::Desc::new(name_player, longtext);
        let playertag = tags::PlayerTag;
        Player {
            desc,
            account,
            playertag,
            name,
        }
    }
}
