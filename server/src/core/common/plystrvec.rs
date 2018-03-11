// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use specs;

use core::common::*;

// NOTE this is bad if a lot of players have deleted theire account, need to rewrite it later
pub type PlayerStructVec = Vec<Vec<StructInd>>;
pub struct Playerstructvec(PlayerStructVec);

// TODO implement PlayerStructVec #newfunctions
impl Playerstructvec {
    // new structure
    pub fn new() -> PlayerStructVec {
        let v: PlayerStructVec = vec![vec![]];
        v
    }

    // add station for user
    pub fn add_station(&self, player: PlayerInd) {
        println!("test",);
    }
}
// remove station for user
// check if there is a station for this user
// add new user
