/// Station entity related functions
use crate::core::component::*;
use crate::core::common::formulars::*;
use bevy::prelude::*;
use std::fmt::Display;

#[derive(Bundle, Reflect)]
pub struct Station {
    desc: desc::Desc,
    name: Name,
    basics: basics::BasicParameter,
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
        // basics
        let ext = Vec3::new(5.0, 3.0, 3.0);
        let mass = mass_sqm(ext, 5.0);
        let basics = basics::BasicParameter::new(ext, mass);

        // other
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
            basics,
            energy,
            shadow,
            stationtag,
            transform,
            global,
        }
    }
}
