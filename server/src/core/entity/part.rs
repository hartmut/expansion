use crate::core::component::*;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Reflect, Inspectable, Bundle)]
pub struct Part {
    desc: desc::Name,
    basics: basics::BasicParameter,
    energy: energy::Energy,
}

impl Part {
    pub fn create(name: impl Into<String>,ext: Vec3, mass: f32) -> Part {
        let desc = desc::Name::new(name, "");
        let basics = basics::BasicParameter::new(ext, mass);
        let energy = energy::Energy {
            act_storage: 100.0,
            max_storage: 100.0,
            production: 0.0,
            consumption: 10.0,
        };
        Part {
            desc,
            basics,
            energy,
        }
    }
}
