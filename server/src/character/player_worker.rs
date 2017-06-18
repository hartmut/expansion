// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
/// worker for player

use common::workertrait::*;
use common::myuuid::*;
use common::configuration::*;
use std::collections::BTreeMap;
use super::player::Player;

/// this structure holds the informations for the worker in the player area
#[derive(Debug)]
pub struct PlayerWorker {
    /// structure with 'general worker structure'
    worker_struct: WorkerStruct,
    // persistancefile for player
    playerdata: FileData,
    // TODO insert link on configuration instead of copying all the values
    // config: Configuration,
    // Btree with player in it
    player: BTreeMap<ExpUuid, Player>,
}

impl WorkerTrait<PlayerWorker> for PlayerWorker {
    fn new(name: String, config: &Configuration) -> PlayerWorker {

        let btree: BTreeMap<ExpUuid, Player> = BTreeMap::new();

        PlayerWorker {
            worker_struct: WorkerStruct { name: name },
            playerdata: config.get_player_config(),
            player: btree,
        }
    }

    fn send_update(&self) {}

    fn get_update(&mut self) {}

    fn step(&mut self) {
        println!("one step in player worker", );
    }
}
