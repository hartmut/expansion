// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// templates of the modules usabel in stations and ships

// NOTE change to legion

// uses
use serde::{Deserialize, Serialize};
use utils::myuuid::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
    // general informations
    name: String,             // name/description of the module
    energy: i64,              // positiv when energy producing, negativ when energy using
    prod_by_receipe: ExpUuid, /* uuid of receipe with which it had been produced, neede for dismantling of module */
    // production and storage
    cur_prod_receipe: ExpUuid, /* uuid of receipe currently producing anything from food to other modules, later more than one recipe in parallel? */
    // sizing of the module in m
    #[serde(default)]
    xsize: u32,
    #[serde(default)]
    ysize: u32,
    #[serde(default)]
    zsize: u32,
    #[serde(default)]
    mass: u64, // in kg
}

impl Module {
    //! create a new Module for a structure
    pub fn new(
        name: String,
        station_uuid: ExpUuid,
        energy: i64,
        prod_by_receipe: ExpUuid,
    ) -> Module {
        Module {
            prod_by_receipe: prod_by_receipe,
            name: name,
            energy: energy,
            cur_prod_receipe: ExpUuid::nil(),
            xsize: 1,
            ysize: 1,
            zsize: 1,
            mass: 0,
        }
    }
}
