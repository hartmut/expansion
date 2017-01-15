// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
//! contains information about basic elements and raw materials
//! or should we model the basic elements into components?

//! includes all the elements. And later when needed the isotops, like HE3 and Uxxx

// uses
use common::myuuid;

//! all the elemenets
// updates from https://raw.githubusercontent.com/Bowserinator/Periodic-Table-JSON/master/PeriodicTableJSON.json

// TODO design struct and read from file #NEXT
// IDEA move this to the structure module? do I need this data anywhere else?

struct material { //TODO integrate in components?
    //TODO make ExpUuid determinstic for elements from the periodic table, probpably use from_fields or from_bytes
    uuid: ExpUuid,   // global id of the raw material
    name: String,    // Name of the raw material
}

//TODO create collection struct to reference to, use arc?

//TODO documentation doesn't work
///
///print the content of a file
///
///# Returns
///
///* nothing
///
///# Arguments
///
///* `f` File to print, expects a BufReader<File>
///

//TODO configuration in TOML file, and load the data the structure worker?

//TODO stdstruct?, read all elements from local json databasefile

//TODO update function for database from web

//TODO search in database
