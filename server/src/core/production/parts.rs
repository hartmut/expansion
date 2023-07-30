// Experimental Simulator of a cooperative solar system economy.
// contains informations about parts

// uses
use bevy::prelude::*;

// one Part
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct Part {
    pub id: u64,      /* id of this part and also of receip with which
                      Componentd had been produced, usefull for dismantling */
    pub name: String, // Name of the part
    pub weight: f32,  // in kg of one item
    pub volume: f32, /* in m^3 of one item, at first we ignore the case that the volume could be 1 m^3 but
                     it is realy long and doesn't fit into the bay */
}

// a Bundle of parts
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct PartBundle {
    amount: u64, // how much
    part: Part,  // parts
}

impl Default for PartBundle {
    fn default() -> PartBundle {
        PartBundle {
            amount: 0,
            part: Part::default(),
        }
    }
}

impl Default for Part {
    fn default() -> Part {
        Part {
            id: 0,
            name: "".to_string(),
            weight: 0.0,
            volume: 0.0,
        }
    }
}
