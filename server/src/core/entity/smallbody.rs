// Copyright (C) 2016  Hartmut Prochaska
// Experimental Simulator of a cooperative solar system economy.
// See doc/LICENSE for licensing information
use amethyst::prelude::*;
use csv::{Reader, ReaderBuilder, Trim};
use serde::Deserialize;

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

fn default_string() -> String {
    "".to_string()
}

fn default_f32() -> f32 {
    0.0
}

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

#[derive(Debug, Deserialize)]
struct Orbits {
    num: u32,
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

// TODO implement asteroid import
impl SmallBody {
    pub fn create(_world: &mut World) {
        // read bodies data
        let mut rdr = Reader::from_path("assets/bodiesdata.csv".to_string()).unwrap();
        for result in rdr.deserialize() {
            let record: Bodies = result.unwrap();
            println!("{:?}", record);
        }

        // read orbits
        let mut rdr = ReaderBuilder::new()
            .delimiter(b',')
            .trim(Trim::All)
            .from_path("assets/bodiesorbit.csv".to_string())
            .unwrap();
        for result in rdr.deserialize() {
            let _record: Orbits = result.unwrap();
        }

        // mix this files and create the asteroid entities
    }
}
