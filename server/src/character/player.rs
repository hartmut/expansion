// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// player code, independent whether it is a NPC or a PC

sruct player {
    UUId:   u64,        // uuid of the player
    name: String,       // Name of the player
    FactionID: u64,     // take part in faction x, at first 0, later to modell
    credits: u64,       // credits in purse
    // TODO list of stations and ships


}
