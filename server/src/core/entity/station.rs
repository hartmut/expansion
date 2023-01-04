/// Station entity related functions
use crate::core::component::*;
use bevy::prelude::*;
use std::fmt::Display;

#[derive(Bundle, Reflect)]
pub struct Station {
    desc: desc::Desc,
    name: Name,
    energy: energy::Energy,
    shadow: shadow::Shadow,
    stationtag: tags::StationTag,
    transform: Transform,
    global: GlobalTransform,
}

/// a station has the following parts
/// - a description
/// - an owner as a parent
/// - modules as children which are arranged in a matrix

impl Station {
    pub fn create(name_station: impl Into<String> + Clone + Display) -> Station {
        let name_entity = name_station.clone().to_string();
        let desc = desc::Desc::new(name_station, "");
        let name = Name::new(name_entity);
        let energy = energy::Energy::default();
        let shadow = shadow::Shadow::default();
        let stationtag = tags::StationTag;
        // TODO create from from position in space (fn from_xyz)
        let transform = Transform::default();
        let global = GlobalTransform::default();
        Station {
            desc,
            name,
            energy,
            shadow,
            stationtag,
            transform,
            global,
        }
    }
}
