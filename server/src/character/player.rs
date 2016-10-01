// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
//! player code, only for PCs

// uses
use common::stdtrait::StdTrait;
use serde_json;
use common::myuuid;

// players data

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    uuid: myuuid::ExpUuid,          // uuid of the player
    name: String,       // Name of the player
    credits: u64,       // credits in purse, needed for buying inventar and material
    // list of available recipes of this player

    // TODO list of stations and ships uuids, send messages to update states?

}

impl Player {
    pub fn new ( name: String ) -> Player {
        Player {
            uuid: myuuid::get_new_uuid(),
            name: name,
            credits: 0,
        }
    }
}


impl StdTrait<Player> for Player {
    fn update(&mut self) {
        self.credits += 1;
    }

    fn serialize (&self) -> String
    {
        serde_json::to_string(&self).unwrap()
    }

    fn new_from_deserialized (input: &String) -> Player
    {
        serde_json::from_str(&input).unwrap()
    }

}
