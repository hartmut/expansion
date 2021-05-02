// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// player code, only for PCs

// uses
use serde::{Deserialize, Serialize};
use utils::myuuid::*;

// players resources

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    uuid: ExpUuid, // uuid of the player
    name: String,  // Name of the player
    credits: u64,  /* credits in purse, needed for buying inventar and material
                    * list of available recipes of this player
                    *
                    * list of stations and ships uuids, send messages to update states? */
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            uuid: get_new_uuid(),
            name: name,
            credits: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_player_example() {
        use utils::fileoperations::*;

        // create a standard module
        let _new_player1 = Player {
            uuid: get_new_uuid(),
            name: "Ian Banks".to_string(),
            credits: 100,
        };

        let _new_player2 = Player {
            uuid: get_new_uuid(),
            name: "Daniel Suarez".to_string(),
            credits: 1000,
        };

        // and now write it
        let mut g = newlinewriter("src/tests/testdataout/playertestout.json".to_string());
        closefile(&mut g);
    }
}
