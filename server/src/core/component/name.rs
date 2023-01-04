// Experimental Simulator of a cooperative solar system economy.
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

// Descriptions
#[derive(Inspectable, Clone, Debug, PartialEq, Eq, Reflect, Default, Component)]
#[reflect(Component)]
pub struct Name {
    pub name: String,
    pub longtext: String,
}

impl Name {
    pub fn new(name: impl Into<String>, longtext: impl Into<String>) -> Self {
        let name = name.into();
        let longtext = longtext.into();
        Name { name, longtext }
    }

}