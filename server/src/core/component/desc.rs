// Experimental Simulator of a cooperative solar system economy.
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

// COMEBAK inspection doesn't work
// Descriptions
#[derive(Clone, Debug, PartialEq, Reflect, Default, Inspectable)]
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
