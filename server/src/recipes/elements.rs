// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// contains information about basic elements and raw materials
// or should we model the basic elements into components?

//! includes all the elements. And later when needed the isotops, like HE3 and Uxxx

// uses
use serde_json::Error;
use serde_json;
use serde::de::{Deserialize, Deserializer};
use common::fileoperations::*;

// all the elemenets
// updates from https://github.com/Bowserinator/Periodic-Table-JSON/blob/master/PeriodicTableJSON.json

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Element {
    #[serde(deserialize_with="parse_string")]
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
    #[serde(deserialize_with="parse_string")]
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

    // read the json file and convert it to a vector of elements
    let result = read_file_to_string("src/data/PeriodicTableJSON-cleaned.json".to_string());
    let e: Result<ElementListVec, Error> = serde_json::from_str(&result);

    // check if the conversion of the elementlist from the json file worked as predicted
    let evec: ElementListVec = match e {
        Ok(elementlist) => elementlist,
        Err(error) => {
            panic!("somethings is wrong with the deserialization of the elementsfile: {:?}",
                   error);
        }
    };

    // define dark matter for element 0
    let dark_matter = Element {
        name: "Dark Matter".to_string(),
        appearance: "everywhere, but you cant see it".to_string(),
        atomic_mass: 0.0,
        boil: 0.0,
        category: "unobtainium".to_string(),
        color: "black".to_string(),
        density: 0.0,
        discovered_by: "no one yet".to_string(),
        melt: 0.0,
        molar_heat: 0.0,
        named_by: "need to find out".to_string(),
        number: "0".to_string(),
        period: 0,
        phase: "dark matter".to_string(),
        source: "https://en.wikipedia.org/wiki/Dark_matter".to_string(),
        spectral_img: "".to_string(),
        summary: "Dark matter is a hypothetical type of matter distinct from baryonic matter \
                  (ordinary matter such as protons and neutrons), neutrinos and dark energy. The \
                  existence of dark matter would explain a number of otherwise puzzling \
                  astronomical observations.The name refers to the fact that it does not emit or \
                  interact with electromagnetic radiation, such as light, and is thus invisible \
                  to the entire electromagnetic spectrum. Although dark matter has not been \
                  directly observed, its existence and properties are inferred from its \
                  gravitational effects such as the motions of visible matter,gravitational \
                  lensing, its influence on the universe's large-scale structure, on galaxies, \
                  and its effects in the cosmic microwave background."
            .to_string(),
        symbol: "not yet".to_string(),
        xpos: 18,
        ypos: 1,
    };

    evec

}

// TODO create collection struct to reference to, use arc?

// TODO configuration in TOML file, and load the data the structure worker?

// TODO stdstruct?, read all elements from local json databasefile

// TODO update function for database from web

// TODO search in database
