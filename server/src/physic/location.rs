// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
//! calculations of positions in solar system

// pos based on Ecliptic_coordinate_system (wikipedia)
// TODO known objects like Jupiter should be determined by a standard library, use astro-rust?

pub struct SpaceObj {
    long: f64, // longtitude (l) in relation to sun
    lat:  f64, // latitude (b) in relation to sun
    dist: f64, // distance from sun (r) in AU
    mass: u64, // in kg, needed for accelerations
    // center of world is sun, jupiter, earth ? default should be sun
    // worldtime: time::PreciseTime,      // time in the world TODO coud be calculated from tick
}

impl SpaceObj {
    pub fn new (longtitude: f64, latitude: f64, distance: f64, mass: u64) -> SpaceObj {
        SpaceObj {
            long: longtitude,
            lat: latitude,
            dist: distance,
            mass: mass,
            // worldtime - determine by using tick necessary?
        }
    }
}
