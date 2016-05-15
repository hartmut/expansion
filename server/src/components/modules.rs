// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
//! templates of the modules usabel in stations and ships
//! TODO how to implement improvements etc.? copy module an work with this in runtime?

// uses
use uuid::Uuid;
use rustc-serialize::json::{self, ToJson, Json};

struct module {
//general informations
    UUId:   u64,      // global id of the module
    name: String,     // name/description of the module
    energyprod: u64, // energy production per tick -> energy module?
    energyuse: u64,  // energy use per tick -> energy module?

//production and storage
    current_receipe: u64, // uuid of receipe currently producing anything from food to other modules


// seldom use
    prod_by_receipe: u64, //uuid of receipe with which it had been produced, usefull for dismantling of module


}
