/// Station entity related functions
use crate::core::component::desc::Desc;
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
    pub fn create(name: impl Into<String>) -> Station {
        let desc = Desc::new(name, "");
        Station { desc }
    }
    
    // pub fn create1(&mut commands: Commands, name: impl Into<String>) -> Station {
    //     let desc = Desc::new(name, "");
    //     let station = Station { desc };
    //     commands.spawn().insert_bundle(station).id();
    // }
}
