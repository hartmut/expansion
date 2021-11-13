/// Station entity related functions
use crate::core::component::desc::Desc;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

#[derive(Bundle, Reflect)]
pub struct Station {
    desc: Desc,
}

/// a station has the following parts
/// - a description
/// - an owner as a parent
/// - modules as children which are arranged in a matrix

impl Station {
    pub fn create(mut commands: EntityCommands, name: impl Into<String>) -> Entity {
        let desc = Desc::new(name, "");
        commands.insert_bundle(Station { desc}).id()
    }
}
