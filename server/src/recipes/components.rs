// Experimental Simulator of a cooperative solar system economy. Copyright (C) 2016  Hartmut
// Prochaska See doc/LICENSE for licensing information
// contains informations about components
// needed to build stations, other components, etc.

// NOTE change to legion

// uses
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use utils::myuuid::*;

// one Component
#[derive(Serialize, Deserialize, Debug)]
pub struct Component {
    pub uuid: ExpUuid,                 // id of the component
    pub name: String,                  // Name of the component
    pub weight: f64,                   // in kg of one item
    pub volume: f64, /* in m^3 of one item, at first we ignore the case that the volume could be 1 m^3 but it is realy long and doesn't fit into the bay */
    pub prodfrom_recipe_uuid: ExpUuid, /* UUId of receip with whichComponentd had been produced, usefull for dismantling */
}

pub type ComponentListVec = BTreeMap<ExpUuid, Component>;
