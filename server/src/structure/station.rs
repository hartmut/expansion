// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// central relevant informations for a station, station is also usable for spacecraft, a station
// can have thrust

// uses
use serde_json;
use serde_json::error::Error;
use std::collections::BTreeMap;
use common::stdtrait::StdTrait;
use common::myuuid::*;
use physic::location::SpaceObj;
use super::modules::Module;

#[derive(Serialize, Deserialize, Debug)]
pub struct AStation {
    uuid: ExpUuid, // global uniqe id, describes this
    name: String, // name of this station
    owner: ExpUuid, // player who owns this station
    energyuse: u64, // energy usage per tick, sum over all modules
    energyprod: u64, // energy production per tick, sum over all modules
    personcount: u64, // how many people are on this station
    o2prod: u64, // production of O2, see above -> people module?
    o2use: u64, // use of O2 for people TODO modelle by a prduction modules -> people module?
    #[serde(default)]
    mass: u64, // mass of the station
    location: SpaceObj, // where am I?
    module_list: Vec<Module>, /* list of modules of this station
                               *
                               * list of inventar on this station
                               * list of NPC on station */
}

// general BtreeMap type for stations
pub type BtreeStations = BTreeMap<ExpUuid, AStation>;

// TODO create matrix of modules, one meter is one cell? or linked list dijkstra style? transpotision
// not necessesary, but difficult later to display, optimization later :)

impl AStation {
    pub fn new(name: String, owner: ExpUuid) -> AStation {

        let module_list_temp: Vec<Module> = Vec::new();

        AStation {
            uuid: get_new_uuid(),
            name: name,
            owner: owner,
            energyuse: 0,
            energyprod: 0,
            personcount: 0,
            o2use: 0,
            o2prod: 0,
            mass: 0,
            location: SpaceObj::new(1.0, 12.0, 3.0, 4),
            module_list: module_list_temp,
        }

        // TODO add a standard module for persons with minimal functionality,
        // TODO need a recipe for a minimal module
    }

    // TODO integrate into stdtraits
    pub fn deserialize_test(input: &String) -> Result<AStation, Error> {
        serde_json::from_str(&input)
    }

    pub fn arriving_people(&self) {
        // error if not enough place on station
        // check for o2 and throw error if not enough o2 available
    }

    pub fn departing_people(&self) {
        // error if not enogh people on station
    }

    pub fn attach_module(&self, definition: String) {
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

impl StdTrait<AStation> for AStation {
    fn getuuid(&self) -> ExpUuid {
        self.uuid
    }

    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn new_from_deserialized(input: &String) -> AStation {
        serde_json::from_str(&input).unwrap()
    }

    fn step(&mut self) {
        // iterate over modules and update local variables

        // update player, send infos relvant for player to player structure
        unimplemented!()
    }
}
