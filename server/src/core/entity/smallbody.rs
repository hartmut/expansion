// Copyright (C) 2016  Hartmut Prochaska
// Experimental Simulator of a cooperative solar system economy.
// See doc/LICENSE for licensing information
use amethyst::prelude::*;

pub struct SmallBody;

/* Astronomical Database:
* download https://ssd.jpl.nasa.gov/?sb_elem the "numbered Asteroids" list
* enrich it with physical parameters with selection of size > 0.1km
    into assets/smallbodies.csv. At first select " object fullname" (number and  Name) to
    have an anchor.
* more data could propably be found at https://www.asterank.com/
    for example minerals and checmical elements
*/

// TODO implement asteroid import etc.
impl SmallBody {
    pub fn new(world: &mut World) {}
}
