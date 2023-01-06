use crate::core::component::*;
use bevy::prelude::*;
use std::fmt::Display;

#[derive(Reflect, Bundle)]
pub struct Part {
    desc: desc::Desc,
    name: Name,
    basics: basics::BasicParameter,
    energy: energy::Energy,
}

impl Part {
    pub fn create(name_part: impl Into<String>+ Clone + Display,ext: Vec3, mass: f32) -> Part {
        let name_entity = name_part.clone().to_string();
        let desc = desc::Desc::new(name_part, "");
        let name = Name::new(name_entity);
        let basics = basics::BasicParameter::new(ext, mass);
        let energy = energy::Energy {
            act_storage: 100.0,
            max_storage: 100.0,
            production: 0.0,
            consumption: 10.0,
        };
        Part {
            desc,
            name,
            basics,
            energy,
        }
    }
}
