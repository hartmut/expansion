// Experimental Simulator of a cooperative solar system economy.
use bevy::prelude::*;

/*
when used in a station the vector will be empty and only a total count will be managed
*/

// TODO adapt NPC design, simulate people

// Descriptions
#[derive(Clone, Debug, PartialEq, Eq, Reflect, Default, Component)]
#[reflect(Component)]
pub struct People {
    pub people_count: u32,
}

impl People {
    pub fn new(people_count: u32) -> Self {
        People { people_count }
    }
}
