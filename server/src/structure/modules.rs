// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
//! templates of the modules usabel in stations and ships
//! TODO how to implement improvements etc.? copy module and work with this in runtime?

// uses
use common::myuuid::*;
use common::stdtrait::StdTrait;
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
//general informations
    uuid: ExpUuid,      // global id of this module
    name: String,       // name/description of the module
    energy: u64,        // positiv when energy using, negativ when energy producing

//production and storage
    cur_prod_receipe: ExpUuid, // uuid of receipe currently producing anything from food to other modules
    // type of things it can store, vector of ...

// seldom use
    prod_by_receipe: ExpUuid, //uuid of receipe with which it had been produced, usefull for dismantling of module

}

impl StdTrait<Module> for Module {
    fn update(&mut self) {
    }

    fn serialize (&self) -> String
    {
        serde_json::to_string(&self).unwrap()
    }

    fn new_from_deserialized (input: &String) -> Module
    {
        serde_json::from_str(&input).unwrap()
    }

}
