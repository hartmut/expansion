// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
/// worker for player

use common::workertrait::*;
use common::myuuid::*;
use std::collections::BTreeMap;
use super::player::Player;

/// this structure holds the informations for the worker in the player area
#[derive(Debug)]
pub struct PlayerWorker {
    /// structure with 'general worker structure'
    worker_struct: WorkerStruct,
    // persistancefile for player
    playerfile: String,
    // Btree with player in it
    player: BTreeMap<ExpUuid, Player>,
}

impl WorkerTrait<PlayerWorker> for PlayerWorker {
    fn new(name: String, filename: String) -> PlayerWorker {

        let btree: BTreeMap<ExpUuid, Player> = BTreeMap::new();

        PlayerWorker {
            worker_struct: WorkerStruct { name: name },
            playerfile: filename,
            player: btree,
        }
    }

    fn initialize(&mut self) {
        // TODO fill initialization analog StructureWorker
    }

    fn run(&mut self) {}

    fn send_update(&self) {}

    fn get_update(&mut self) {}
}
