use crate::core::component::desc::Desc;
use crate::core::component::basics::BasicParameter;
use bevy_inspector_egui::Inspectable;
use bevy::prelude::*;

#[derive(Bundle, Reflect, Inspectable)]
pub struct Module{
    desc: Desc,
    basics: BasicParameter,
}

impl Module {
    pub fn create(
        name: impl Into<String>,
    ) -> Module {
        let desc = Desc::new(name, "");
        let extend = Vec3::new(5.0,3.0,3.0);
        let basics = BasicParameter{
             mass: 1000.0,
             usablevol: 5.0,
             outervol: 6.0,
             extend,
        };
        Module { desc, basics }
    }
}
