// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use specs;
use std::collections::BTreeMap;

use core::common::*;

pub struct PlayerStructure {
    player: specs::Index,
    structure: specs::Index,
}

// pub type PlayerInd = specs::Index;
// pub type StructInd = specs::Index;
pub type StructPlayerMap = BTreeMap<StructInd, PlayerInd>;
