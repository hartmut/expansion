// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
//! player code, independent whether it is a NPC or a PC

// uses
use common::myuuid;
use common::traits::StdTrait;
use serde_json;

// players data

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    uuid: myuuid::ExpUuid,          // uuid of the player
    name: String,       // Name of the player
    credits: u64,       // credits in purse

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

    fn deserialize (&self, input: &String) -> Player
    {
        serde_json::from_str(&input).unwrap()
    }

}
