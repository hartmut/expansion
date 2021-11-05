// Experimental Simulator of a cooperative solar system economy.
use crate::core::component::account::Account;
use crate::core::component::desc::Desc;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Bundle, Inspectable)]
pub struct Player {
    desc: Desc,
    account: Account,
}

/// a player has the following parts
/// - a description,
/// - an account
/// - some stations as as children

impl Player {
    pub fn create(
        mut commands: Commands,
        name: impl Into<String>,
        longtext: impl Into<String>,
    ) -> Entity {
        let account = Account::new(1000);
        let desc = Desc::new(name, longtext);
        commands
            .spawn()
            .insert_bundle(Player { desc, account })
            .id()
    }
}
