// Copyright (C) 2016  Hartmut Prochaska
// Experimental Simulator of a cooperative solar system economy.
// See doc/LICENSE for licensing information
use amethyst::prelude::*;

pub struct SmallBody;

/* Astronomical Database:
* download https://ssd.jpl.nasa.gov/?sb_elem the "numbered Asteroids" list and modify it to a
    comma separated csv file 
* enrich it with physical parameters with selection of size > 0.1km
    into assets/smallbodies.csv. At first select " object fullname" (number and  Name) fto
    have an anchor.
* more data could propably be found at https://www.asterank.com/
    for example minerals and checmical elements
*/

// TODO implement asteroid import etc.
impl SmallBody {
    pub fn new(world: &mut World) {
        // read ELEMENTS.number

        // read smallbodies.csv

        // mix this files and create the asteroid entities
    }
}
