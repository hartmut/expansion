// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// central relevant informations for a station, station is also usable for spacecraft, a station
// can have thrust

// uses
use utils::myuuid::*;

#[derive(Debug)]
pub struct AStation {
    name: String,       // name of this station
    owner: ExpUuid,     // player who owns this station
    energyuse: u64,     // energy usage per tick, sum over all modules
    energyprod: u64,    // energy production per tick, sum over all modules
    personcount: u64,   // how many people are on this station
    o2prod: u64,        // production of O2, see above -> people module?
    o2use: u64,         // use of O2 for people
    mass: u64,          // mass of the station
                         /*
                         * list of inventar on this station -> stored in storagemodules
                         * list of NPC on station */
}

// TODO create matrix of modules
// not necessesary, but difficult later to display, optimization later :)

impl AStation {
    pub fn new(name: String, owner: ExpUuid) -> AStation {

        AStation {
            name: name,
            owner: owner,
            energyuse: 0,
            energyprod: 0,
            personcount: 0,
            o2use: 0,
            o2prod: 0,
            mass: 0,
        }
    }

    pub fn arriving_people(&self) {
        // error if not enough place on station
        // check for o2 and throw error if not enough o2 available
    }

    pub fn departing_people(&self) {
        // error if not enogh people on station
    }

    pub fn attach_module(&self, definition: String) {
        println!("{:?}", definition);
        // how long does it take to attach it?
        // definition contains json definition of new module
        // create module from definition
        // update energyuse/energyprod
        // insert into module list
    }

    pub fn detach_module(&self) {
        // how long does it take to detach it?
        // remove from module list
        // update energyuse/energyprod
        // what to do with the module after detaching it
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}
