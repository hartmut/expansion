// Experimental Simulator of a cooperative solar system economy.
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

// Money
#[derive(Clone, Debug, PartialEq, Eq, Reflect, Component, Default)]
#[reflect(Component)]
pub struct Character {
    credits: u64,
    // COMEBACK objects owned by this Character 
    owns: Vec<Entity>,
}

impl Character {
    pub fn new(credits: u64) -> Self {
        Character { credits, owns: vec![] }
    }
}