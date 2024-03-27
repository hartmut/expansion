use std::fmt::Display;

// Experimental Simulator of a cooperative solar system economy.
use crate::core::component::*;
use bevy::prelude::*;
use moonshine_save::prelude::*;

#[derive(Bundle)]
pub struct Player {
    name: Name,
    desc: desc::Desc,
    account: character::Character,
    playertag: tags::PlayerTag,
    save: Save, // automatic saving
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
        let save = Save::default();
        Player {
            desc,
            account,
            playertag,
            name,
            save,
        }
    }
}
