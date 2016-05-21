// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
//! contains information about basic elements and raw materials
//! or should we model the basic elements into components?

//! includes all the elements. And later when needed the isotops, like HE3 and Uxxx

// uses
use common::myuuid;

//! all the atomar material you can't build

struct rawmaterial { //TODO integrate in components?
    uuid: ExpUuid,      // global id of the raw material
    name: String,     // Name of the raw material

}
