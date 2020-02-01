// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// calculations of positions in solar system

// pos based on Ecliptic_coordinate_system (wikipedia)

use serde::{Deserialize, Serialize};
use specs;

// TODO other name, as mass is not a part of location but a part of a ...?
#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    long: f64, // longtitude (l) in relation to sun
    lat: f64,  // latitude (b) in relation to sun
    dist: f64, // distance from sun (r) in AU
    mass: u64, /* in kg, needed for accelerations
                * center of world is sun, jupiter, earth ? default should be sun */
}

impl specs::Component for Location {
    type Storage = specs::VecStorage<Location>;
}

impl Location {
    pub fn new(longtitude: f64, latitude: f64, distance: f64, mass: u64) -> Location {
        Location {
            long: longtitude,
            lat: latitude,
            dist: distance,
            mass: mass,
        }
    }
}
