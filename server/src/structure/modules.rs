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
    staion_uuid: ExpUuid, // Uuid of the station for later reference
    name: String,       // name/description of the module
    energy: i64,        // positiv when energy using, negativ when energy producing
    prod_by_receipe: ExpUuid, //uuid of receipe with which it had been produced, neede for dismantling of module

//production and storage
    cur_prod_receipe: ExpUuid, // uuid of receipe currently producing anything from food to other modules, later more than one recipe in parallel?

    // type of things it can store, vector of ...

}

impl Module {

    //! create a new Space Module
    pub fn new(name: String, station_uuid: ExpUuid, energy: i64, prod_by_receipe: ExpUuid) -> Module {

        Module {
            uuid: get_new_uuid(),
            staion_uuid: station_uuid,
            name: name,
            energy: energy,
            prod_by_receipe: prod_by_receipe,
            cur_prod_receipe: ExpUuid::nil(),
        }
    }

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
