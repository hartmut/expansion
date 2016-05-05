// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// central relevant informations for a station, station is also usable for spacecraft, a station
// can have thrust

struct station {
    UUId: u64,  // global uniqe id, describes this station
    name: String,  // name of this station
    cost: u64, //cost per tick to keep it running
    energyprod: u64, // energy production per tick
    energyuse: u64,  // energy use per tick
    //list of people on station
    //list of components of station
}
