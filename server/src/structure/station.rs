// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
//! central relevant informations for a station, station is also usable for spacecraft, a station
//! can have thrust

// uses
use common::myuuid;
use common::traits::StdTrait;
use physic::location::SpaceObj;
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct AStation {
    uuid: myuuid::ExpUuid,       // global uniqe id, describes this station
    name: String,    // name of this station
    cost: u64,       // cost per tick to keep it running, or is it just energy? who pays the people? -> people module?
    energyprod: u64, // energy production per tick -> energy module?
    energyuse: u64,  // energy use per tick -> energy module?
    o2prod: u64,     // production of O2, see above -> people module?
    o2use: u64,      // use of O2 for people TODO check if it can be modelled by a prduction modules -> people module?
    location: SpaceObj, //where am I?

    //storage room of type x
    //list of people on station
    //list of inventar on this station
    //list of modules of this station
}

impl AStation {
    pub fn new ( name: String ) -> AStation {
        AStation {
            uuid: myuuid::get_new_uuid(),
            name: name,
            cost: 0,
            energyprod: 0,
            energyuse: 0,
            o2use: 0,
            o2prod: 0,
            location: SpaceObj::new(1.0, 12.0 ,3.0 ,4),
        }
    }
}

impl StdTrait<AStation> for AStation {
    fn update(&mut self) {
        self.cost += 1;
    }

    fn serialize (&self) -> String
    {
        serde_json::to_string(&self).unwrap()
    }

    fn deserialize (&self, input: &String) -> AStation
    {
        serde_json::from_str(&input).unwrap()
    }

}
