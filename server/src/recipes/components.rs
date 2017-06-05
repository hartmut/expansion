// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// contains informations about components needed to build stations, other components, etc.

// IDEA do I need this or will I only need materials?
// COMBAK work on this next time

// uses
use serde_json::Error;
use serde_json;
use serde::de::{Deserialize, Deserializer};
use std::collections::BTreeMap;
use common::stdtrait::StdTrait;
use common::myuuid::*;

// one material
#[derive(Serialize, Deserialize, Debug)]
pub struct Component {
    uuid: ExpUuid, // id of the component
    name: String, // Name of the component
    weight: u64, // in kg of one item
    volume: u64, /* in m^3 of one item, at first we ignore the case that the volume could be 1 m^3 but it is realy long and doesn't fit into the bay */
    receipe_uuid: ExpUuid, // UUId of receip with whichComponentd been produced, usefull for dismantling
}

pub type ComponentListVec = BTreeMap<ExpUuid, Component>;

impl StdTrait<Component> for Component {
    fn new_from_deserialized(input: &String) -> Component {
        serde_json::from_str(&input).unwrap()
    }

    fn step(&mut self) {
        println!("shouldn't do anything per step", );
    }

    fn getuuid(&self) -> ExpUuid {
        self.uuid
    }

    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
