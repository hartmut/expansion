// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// calculations of positions in solar system

extern crate time;

// pos based on Ecliptic_coordinate_system (wikipedia)
// TODO known objects like Jupiter should be determined by a standard library, use astro-rust?
pub struct Spaceobj {
    long: f64, // longtitude (l) in relation to sun
    lat:  f64, // latitude (b) in relation to sun
    dist: f64, // distance from sun (r) in AU
    mass: u64, // in kg, needed for accelerations
    tick: u64, // last tick when this location had been updated
    worldtime: time::PreciseTime,      // time in the world TODO coud be calculated from tick
}

impl Spaceobj {
    pub fn new (longtitude: f64, latitude: f64, distance: f64, mass: u64, tick: u64)
    -> Spaceobj {
        Spaceobj {
            long: longtitude,
            lat: latitude,
            dist: distance,
            mass: mass,
            tick: tick,
            worldtime: time::PreciseTime::now(),
        }
    }
}
