// Experimental Simulator of a cooperative solar system economy.
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

// Money
#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect, Inspectable, Component, Default)]
#[reflect(Component)]
pub struct Account {
    credits: u64,
}

impl Account {
    pub fn new(credits: u64) -> Self {
        Account { credits }
    }
}
