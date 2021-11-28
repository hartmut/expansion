use crate::core::component::*;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Bundle, Reflect, Inspectable)]
pub struct Module {
    desc: desc::Desc,
    basics: basics::BasicParameter,
    energy: energy::Energy,
}

impl Module {
    pub fn create(name: impl Into<String>) -> Module {
        let desc = desc::Desc::new(name, "");
        let extend = Vec3::new(5.0, 3.0, 3.0);
        let basics = basics::BasicParameter {
            mass: 2000.0,
            volume: 5.0,
            extend,
        };
        let energy = energy::Energy {
            act_storage: 10000.0,
            max_storage: 100000.0,
            production: 10.0,
            consumption: 10.0,
        };
        Module {
            desc,
            basics,
            energy,
        }
    }
}
