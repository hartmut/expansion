// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// contains information about basic elements and raw materials
// or should we model the basic elements into components?

//! includes all the elements. And later when needed the isotops, like HE3 and Uxxx

// uses
use std::sync::Arc;
use serde_json;
use serde::de;
use common::myuuid::*;
use common::fileoperations::*;

// all the elemenets
// updates from https://raw.githubusercontent.com/Bowserinator/Periodic-Table-JSON/master/PeriodicTableJSON.json

// TODO design struct and read from file #NEXT
// IDEA move this to the structure module? do I need this data anywhere else?

#[derive(Serialize, Deserialize, Debug)]
pub struct Element {
    name: String,
    appearance: String,
    atomic_mass: f64,
    boil: f64, // in Kelvin
    category: String,
    color: String,
    density: f64,
    discovered_by: String,
    melt: f64, // in Kelvin
    molar_heat: f64, // in Kelvin
    named_by: String,
    number: String, // atomic number
    period: u32,
    phase: String,
    source: String,
    spectral_img: String,
    summary: String,
    symbol: String,
    xpos: u32,
    ypos: u32,
}

pub type ElementListVec = Vec<Element>;

// read Elementlist from file
pub fn read_elementlist_file() -> ElementListVec {

    let result = read_file_to_string("src/data/PeriodicTableJSON1.json".to_string());
    let result: ElementListVec = serde_json::from_str(&result).unwrap();
    // let result = match checker_elementlist {
    //     Ok(elementlist) => elementlist,
    //     Err(error) => {
    //         panic!("somethings is wrong with the deserelization of the elementsfile: {:?}",
    //                error)
    //     }
    // };
    // elementlist
    result
}


// TODO create collection struct to reference to, use arc?

// TODO documentation doesn't work

// TODO configuration in TOML file, and load the data the structure worker?

// TODO stdstruct?, read all elements from local json databasefile

// TODO update function for database from web

// TODO search in database
