// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
/// worker for player

use utils::myuuid::*;
use utils::configuration::*;
use std::collections::BTreeMap;
use super::player::Player;

/// this structure holds the informations for the worker in the player area
#[derive(Debug)]
pub struct PlayerWorker {
    // persistancefile for player
    playerdata: FileData,
    // TODO insert link on configuration instead of copying all the values
    // config: Configuration,
    // Btree with player in it
    player: BTreeMap<ExpUuid, Player>,
}
