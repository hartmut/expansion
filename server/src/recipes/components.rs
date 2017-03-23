// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// contains informations about components needed to build stations, other components, etc.

// IDEA do I need this or will I only need materials?

// uses
use common::myuuid;

// structure
struct component {
    uuid: ExpUuid, // id of the component
    name: String, // Name of the component
    receipe_uuid: ExpUuid, // UUId of receip with which this had been produced, usefull for dismantling
}
