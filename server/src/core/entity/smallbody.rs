// Copyright (C) 2016  Hartmut Prochaska
// Experimental Simulator of a cooperative solar system economy.
// See doc/LICENSE for licensing information
use crate::utils::fileoperations::read_file_to_string;
use amethyst::prelude::*;
use csv::*;

pub struct SmallBody;

/* Astronomical Database:
* download https://ssd.jpl.nasa.gov/?sb_elem the "numbered Asteroids" list and modify it to a
    comma separated csv file and move it into assets/bodiesorbit.csv
* enrich it with physical parameters with selection of size > 0.1km
    into assets/smallbodies.csv. At first select " object fullname" (number and  Name) fto
    have an anchor.
* more data could propably be found at https://www.asterank.com/
    for example minerals and checmical elements
*/

struct bodies {}
struct orbits {}

// TODO implement asteroid import etc.
impl SmallBody {
    pub fn create(_world: &mut World) {
        let _bodies = read_file_to_string("assets/smallbodies.csv".to_string());
        let _orbits = read_file_to_string("assets/bodiesorbit.csv".to_string());
        // read bodies data

        // read orbits

        // mix this files and create the asteroid entities
    }
}
