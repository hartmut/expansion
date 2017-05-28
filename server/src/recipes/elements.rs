// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// contains information about basic elements and raw materials
// or should we model the basic elements into components?

//! includes all the elements. And later when needed the isotops, like HE3 and Uxxx

// uses
use std::sync::Arc;
use serde_json::{Value, Error};
use serde_json;
use serde::de::{self, Deserialize, Deserializer};
use common::myuuid::*;
use common::fileoperations::*;

// all the elemenets
// updates from https://github.com/Bowserinator/Periodic-Table-JSON/blob/master/PeriodicTableJSON.json

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Element {
    name: String,
    #[serde(deserialize_with="parse_string")]
    appearance: String,
    #[serde(deserialize_with="parse_f64")]
    atomic_mass: f64,
    #[serde(deserialize_with="parse_f64")]
    boil: f64,
    #[serde(deserialize_with="parse_string")]
    category: String,
    #[serde(deserialize_with="parse_string")]
    color: String,
    #[serde(deserialize_with="parse_f64")]
    density: f64,
    #[serde(deserialize_with="parse_string")]
    discovered_by: String,
    #[serde(deserialize_with="parse_f64")]
    melt: f64,
    #[serde(deserialize_with="parse_f64")]
    molar_heat: f64,
    #[serde(deserialize_with="parse_string")]
    named_by: String,
    number: String,
    period: u32,
    #[serde(deserialize_with="parse_string")]
    phase: String,
    #[serde(deserialize_with="parse_string")]
    source: String,
    #[serde(deserialize_with="parse_string")]
    spectral_img: String,
    #[serde(deserialize_with="parse_string")]
    summary: String,
    #[serde(deserialize_with="parse_string")]
    symbol: String,
    xpos: u32,
    ypos: u32,
}

pub type ElementListVec = Vec<Element>;

fn parse_string<'de, D>(d: D) -> Result<String, D::Error>
    where D: Deserializer<'de>
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or("".to_string()))
}

// TODO react correctly to other types off errors, e.g. "" instead of null
fn parse_f64<'de, D>(d: D) -> Result<f64, D::Error>
    where D: Deserializer<'de>
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or(0.0))
}

// read Elementlist from file
// TODO get filename from config
// TODO insert dark matter as element 0

pub fn read_elementlist_file() -> ElementListVec {
    let result = read_file_to_string("src/data/PeriodicTableJSON-cleaned.json".to_string());
    let e: Result<ElementListVec, Error> = serde_json::from_str(&result);

    match e {
        Ok(elementlist) => elementlist,
        Err(error) => {
            panic!("somethings is wrong with the deserialization of the elementsfile: {:?}",
                   error);
        }
    }
}

// TODO create collection struct to reference to, use arc?

// TODO configuration in TOML file, and load the data the structure worker?

// TODO stdstruct?, read all elements from local json databasefile

// TODO update function for database from web

// TODO search in database
