// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// contains information about basic elements and raw materials
// or should we model the basic elements into components?

// includes all the elements. And later when needed the isotops, like HE3 and Uxxx

struct rawmaterial {
    UUId:   u64,      // global id of the rawm aterial
    name: String,     // Name of the raw material

}
