// Experimental Simulator of a cooperative solar system economy. 
// contains informations about parts
// needed to build stations, other parts, etc.

// uses
use serde::{Deserialize, Serialize};

// one Part
#[derive(Serialize, Deserialize, Debug)]
pub struct Component {
    pub name: String,                  // Name of the part
    pub weight: f64,                   // in kg of one item
    pub volume: f64, /* in m^3 of one item, at first we ignore the case that the volume could be 1 m^3 but it is realy long and doesn't fit into the bay */
    pub prodfrom_recipe_uuid: u64, /* Entity of receip with whichComponentd had been produced, usefull for dismantling */
}
