// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
use super::player::Player;
use std::collections::BTreeMap;
use utils::configuration::*;
/// worker for player
use utils::myuuid::*;

/// this structure holds the informations for the worker in the player area
#[derive(Debug)]
pub struct PlayerWorker {
    // persistancefile for player
    playerdata: FileData,
    // config: Configuration,
    // Btree with player in it
    player: BTreeMap<ExpUuid, Player>,
}
