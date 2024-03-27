use crate::core::component::*;
use bevy::prelude::*;
use std::fmt::Display;
use moonshine_save::prelude::*;

#[derive(Bundle)]
pub struct Part {
    name: Name,
    desc: desc::Desc,
    basics: basics::BasicParameter,
    energy: energy::Energy,
    save: Save, // for automatic saving
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
        let save = Save::default();
        Part {
            desc,
            name,
            basics,
            energy,
            save,
        }
    }
}
