// Experimental Simulator of a cooperative solar system economy.
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

/* 
when used in a station the vector will be empty and only a total count will be managed
*/

// Descriptions
#[derive(Inspectable, Clone, Debug, PartialEq, Reflect, Default)]
pub struct People {
    pub people_count: u32,
    // TODO Vector of NPCs, create NPCs first
}

impl People {
    pub fn new(people_count: u32) -> Self {
        People { people_count }
    }
}
