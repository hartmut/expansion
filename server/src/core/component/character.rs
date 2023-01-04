// Experimental Simulator of a cooperative solar system economy.
use bevy::prelude::*;

// Money
#[derive(Clone, Debug, PartialEq, Eq, Reflect, Component, Default)]
#[reflect(Component)]
pub struct Character {
    credits: u64,
    owns: Vec<Entity>,
}

impl Character {
    pub fn new(credits: u64) -> Self {
        Character {
            credits,
            owns: vec![],
        }
    }

    pub fn insert_owned(&mut self, own: Entity) {
        self.owns.push(own);

        // Clean up if the same id had been inserted twice
        self.owns.sort();
        self.owns.dedup();
    }
}
