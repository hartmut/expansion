// Copyright (C) 2016  Hartmut Prochaska
// Experimental Simulator of a cooperative solar system economy.
// See doc/LICENSE for licensing information
use crate::core::component::desc::Desc;
use csv::{Reader, ReaderBuilder, Trim};
use bevy::log::prelude::*;
use serde::Deserialize;
use std::collections::BTreeMap;
use bevy::prelude::*;

// use core::component::asteroidprop::*;

pub struct SmallBody;

/* Astronomical Database:
download https://ssd.jpl.nasa.gov/?sb_elem the "numbered Asteroids" list and modify it to a
    comma separated csv file and move it into assets/bodiesorbit.csv
rename the column "ref" in bodiesorbit.csv to "reference" because of the rust reserved word "ref"

go to https://ssd.jpl.nasa.gov/sbdb_query.cgi
enrich it with physical parameters with selection of size > 0.1km, limit to asteroids and numbered
    into assets/bodiesdata.csv. At first select "object fullname" (number and  Name) to
    have an anchor and then "asteroid-physical"

replace all column titles in both files to lowercase ascci only
*/

#[derive(Debug, Deserialize)]
struct Bodies {
    full_name: String,
    diameter: f32,
    extent: Option<String>,
    albedo: Option<f32>,
    rot_per: Option<f32>,
    gm: Option<f32>,
    bv: Option<f32>,
    ub: Option<f32>,
    ir: Option<String>,
    spec_b: Option<String>,
    spec_t: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct Orbit {
    num: usize,
    name: String,
    epoch: u32,
    a: f32,
    e: f32,
    i: f32,
    w: f32,
    node: f32,
    m: f32,
    h: f32,
    g: f32,
    reference: String,
}

impl Default for Orbit {
    fn default() -> Self {
        Orbit {
            num: 0,
            name: "".to_string(),
            epoch: 0,
            a: 0.0,
            e: 0.0,
            i: 0.0,
            w: 0.0,
            node: 0.0,
            m: 0.0,
            h: 0.0,
            g: 0.0,
            reference: "".to_string(),
        }

    }

}

// TODO implement asteroid import
impl SmallBody {
    pub fn init(_world: &mut World) {
        
        // read orbits
        info!("Reading orbitdata of asteroids");
        let mut orbitmap = BTreeMap::new();
        let mut rdr = ReaderBuilder::new()
        .delimiter(b',')
        .trim(Trim::All)
        .from_path("assets/bodiesorbit.csv".to_string())
        .unwrap();
        for result in rdr.deserialize() {
            let record: Orbit = result.unwrap();
            orbitmap.insert(record.num, record);
        }
        
        // read bodies data
        info!("Reading physical data of asteroids");
        let mut orbit = Orbit::default();
        let mut rdr = Reader::from_path("assets/bodiesdata.csv".to_string()).unwrap();
        for result in rdr.deserialize() {
            let record: Bodies = result.unwrap();
            let (number, _name) = record.full_name.split_at(7);
            let orbitsearch = orbitmap.get(&number.trim().parse::<usize>().unwrap());
            match orbitsearch {
                Some(x) => orbit = x.clone(),
                None => break,
            }
            // mix the files and create the asteroid entities

            // find record from orbitmap

            // create basic, desc, smallbody and orbit components
            let _desc = Desc::new(orbit.name, "");
            // TODO insert extend into basic, and adapt the smallbody component

            // push them to the world
        }
    }
}
