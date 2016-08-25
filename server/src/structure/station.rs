// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
//! central relevant informations for a station, station is also usable for spacecraft, a station
//! can have thrust

// uses
use std::vec;
use serde_json;
use common::stdtrait::StdTrait;
use common::myuuid::*;
use physic::location::SpaceObj;
use components::modules::Module;

#[derive(Serialize, Deserialize, Debug)]
pub struct AStation {
    uuid: ExpUuid,       // global uniqe id, describes this
    name: String,    // name of this station
    owner: ExpUuid,  // player who owns this station
    energy: u64, // positiv energy production and negativ energyuse per tick, sum over all modules
    o2prod: u64,     // production of O2, see above -> people module?
    o2use: u64,      // use of O2 for people TODO modelle by a prduction modules -> people module?
    location: SpaceObj, //where am I?

    //list of modules of this station
    ModuleList: Vec<Module>,
    //list of inventar on this station
    //list of NPC on station
}

impl AStation {
    pub fn new ( name: String ) -> AStation {

        let mut ModuleListTemp: Vec<Module> = Vec::new();

        AStation {
            uuid: get_new_uuid(),
            name: name,
            owner: get_new_uuid(),
            energy: 0,
            o2use: 0,
            o2prod: 0,
            location: SpaceObj::new(1.0, 12.0 ,3.0 ,4),
            ModuleList: ModuleListTemp,
        }
    }

    pub fn getuuid(&self) -> ExpUuid {
        self.uuid
    }
}

impl StdTrait<AStation> for AStation {
    fn update(&mut self) {
    }

    fn serialize (&self) -> String
    {
        serde_json::to_string(&self).unwrap()
    }

    fn new_from_deserialized (input: &String) -> AStation
    {
        serde_json::from_str(&input).unwrap()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use common::stdtrait::StdTrait;

    #[test]
    fn serialize_test() {
        let mut my_station = AStation::new("Firefly".to_string());
        let serialized = my_station.serialize();
        let alternativetempstation: AStation = serde_json::from_str(&serialized).unwrap(); // whats easier - this
    }

}
