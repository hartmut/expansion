use crate::core::common::formulars::*;
use crate::core::component::*;
use bevy::prelude::*;
use std::fmt::Display;

// TODO use https://docs.rs/bevy/0.9.1/bevy/render/prelude/struct.SpatialBundle.html for transforms etc.
#[derive(Bundle, Reflect)]
pub struct Module {
    desc: desc::Desc,
    name: Name,
    basics: basics::BasicParameter,
    energy: energy::Energy,
    shadow: shadow::Shadow,
    moduletag: tags::ModuleTag,
    transform: Transform,
    global: GlobalTransform,
}

impl Module {
    pub fn create(name_module: impl Into<String> + Clone + Display) -> Module {
        let name_entity = name_module.clone().to_string();
        let desc = desc::Desc::new(name_module, "");
        let name = Name::new(name_entity);
        let ext = Vec3::new(5.0, 3.0, 3.0);

        // we will need 5.0kg/sqm mass to hold this together, later on this will be
        // checked on creation and if mass==0 then 5.0 kg/sqm will be assumed
        let mass = mass_sqm(ext, 5.0);

        let basics = basics::BasicParameter::new(ext, mass);
        let energy = energy::Energy {
            act_storage: 10000.0,
            max_storage: 100000.0,
            production: 10.0,
            consumption: 10.0,
        };
        let shadow: shadow::Shadow = Default::default();
        let moduletag = tags::ModuleTag;
        let transform = Transform::default();
        let global = GlobalTransform::default();
        Module {
            desc,
            name,
            basics,
            energy,
            shadow,
            moduletag,
            transform,
            global,
        }
    }

    // TODO implment available volume, insert a part by using a system?
    pub fn get_outer_volume(&self) -> f32 {
        self.basics.get_volume()
    }

    pub fn check_part_volume(volume: f32, outer_volume: f32) -> bool {
        outer_volume > volume
    }
}
