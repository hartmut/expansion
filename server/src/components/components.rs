// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
//! contains informations about components needed to build stations, other components, etc.

// uses
use uuid::Uuid;

// structure
struct component {
    uuid: Uuid,      // id of the component
    name: String,     // Name of the component
    receipe_uuid: Uuid, // UUId of receip with which this had been produced, usefull for dismantling
}