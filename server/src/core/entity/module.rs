use crate::core::component::*;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Bundle, Reflect, Inspectable, Component)]
pub struct Module {
    desc: desc::Desc,
    basics: basics::BasicParameter,
    energy: energy::Energy,
    shadow: shadow::Shadow,
}

impl Module {
    pub fn create(name: impl Into<String>) -> Module {
        let desc = desc::Desc::new(name, "");
        let basics = basics::BasicParameter::new(5.0, 3.0, 3.0, 2000.0);
        let energy = energy::Energy {
            act_storage: 10000.0,
            max_storage: 100000.0,
            production: 10.0,
            consumption: 10.0,
        };
        let shadow: shadow::Shadow = Default::default();
        Module {
            desc,
            basics,
            energy,
            shadow,
        }
    }

    pub fn get_outer_volume(&self) -> f32 {
        self.basics.volume
    }
}


