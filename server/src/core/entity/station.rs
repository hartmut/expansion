/// Station entity related functions
use crate::core::component::*;
use bevy::prelude::*;

#[derive(Bundle, Reflect)]
pub struct Station {
    desc: desc::Desc,
    energy: energy::Energy,
    shadow: shadow::Shadow,
    stationtag: tags::StationTag,
}

/// a station has the following parts
/// - a description
/// - an owner as a parent
/// - modules as children which are arranged in a matrix

impl Station {
    pub fn create(name: impl Into<String>) -> Station {
        let desc = desc::Desc::new(name, "");
        let energy = energy::Energy::default();
        let shadow = shadow::Shadow::default();
        let stationtag = tags::StationTag;
        Station {
            desc,
            energy,
            shadow,
            stationtag,
        }
    }
}
