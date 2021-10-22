// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// contains information about basic elements and raw materials
// or should we model the basic elements into components?

//! includes all the elements. And later when needed the isotops, like HE3 and Uxxx

// uses
use bevy::log::prelude::*;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Error;
use crate::utils::fileoperations::*;

// all the elemenets
// updates from https://github.com/Bowserinator/Periodic-Table-JSON/blob/master/PeriodicTableJSON.json
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Element {
    #[serde(deserialize_with = "parse_string")]
    pub name: String,
    #[serde(deserialize_with = "parse_string")]
    pub appearance: String,
    #[serde(deserialize_with = "parse_f64")]
    pub atomic_mass: f64,
    #[serde(deserialize_with = "parse_f64")]
    pub boil: f64,
    #[serde(deserialize_with = "parse_string")]
    pub category: String,
    #[serde(deserialize_with = "parse_string")]
    pub color: String,
    #[serde(deserialize_with = "parse_f64")]
    pub density: f64,
    #[serde(deserialize_with = "parse_string")]
    pub discovered_by: String,
    #[serde(deserialize_with = "parse_f64")]
    pub melt: f64,
    #[serde(deserialize_with = "parse_f64")]
    pub molar_heat: f64,
    #[serde(deserialize_with = "parse_string")]
    pub named_by: String,
    #[serde(deserialize_with = "parse_string")]
    pub number: String,
    pub period: u32,
    #[serde(deserialize_with = "parse_string")]
    pub phase: String,
    #[serde(deserialize_with = "parse_string")]
    pub source: String,
    #[serde(deserialize_with = "parse_string")]
    pub spectral_img: String,
    #[serde(deserialize_with = "parse_string")]
    pub summary: String,
    #[serde(deserialize_with = "parse_string")]
    pub symbol: String,
    pub xpos: u32,
    pub ypos: u32,
}

type ElementListVec = Vec<Element>;

#[derive(Debug)]
pub struct ElementList {
    elements: ElementListVec,
}

impl Default for ElementList {
    fn default () -> ElementList {
        info!("Reading List of chemical Elements");
        ElementList {
            elements: read_elementlist_file(),
        }
    }
}

impl ElementList {
    pub fn init () -> Self {
        info!("Reading List of chemical Elements");
        ElementList {
            elements: read_elementlist_file(),
        }
    }
}

// read Elementlist from file
fn read_elementlist_file() -> ElementListVec {
    // read the json file and convert it to a vector of elements
    // Source https://raw.githubusercontent.com/Bowserinator/Periodic-Table-JSON/master/PeriodicTableJSON.json
    let result = read_file_to_string("resources/PeriodicTableJSON-cleaned.json".to_string());
    let evec: Result<ElementListVec, Error> = serde_json::from_str(&result);

    // check if the conversion of the elementlist from the json file worked as predicted
    let mut evec: ElementListVec = match evec {
        Ok(elementlist) => elementlist,
        Err(error) => {
            panic!(
                "somethings is wrong with the deserialization of the elementsfile: {:?}",
                error
            );
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
        xpos: 0,
        ypos: 0,
    };

    let mut outevec: ElementListVec = vec![dark_matter];
    outevec.append(&mut evec);
    outevec
}

fn parse_string<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or_else(|| "".to_string()))
}

fn parse_f64<'de, D>(d: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or(0.0))
}
