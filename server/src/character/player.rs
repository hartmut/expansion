// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// player code, independent whether it is a NPC or a PC

sruct player {
    UUId:   u64,        // uuid of the player
    name: String,       // Name of the player
    FactionID: u64,     // take part in faction x
    money: u64,         // creidts in purse
    // TODO list of stations and ships
    
}
