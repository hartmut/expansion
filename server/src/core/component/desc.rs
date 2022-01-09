// Experimental Simulator of a cooperative solar system economy.
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

// Descriptions
#[derive(Inspectable, Clone, Debug, PartialEq, Reflect, Default, Component)]
pub struct Desc {
    pub name: String,
    pub longtext: String,
}

impl Desc {
    pub fn new(name: impl Into<String>, longtext: impl Into<String>) -> Self {
        let name = name.into();
        let longtext = longtext.into();
        Desc { name, longtext }
    }
}
