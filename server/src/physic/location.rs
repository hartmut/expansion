// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// calculations of positions in solar system based on keplers law

extern crate time;

// pos based on Ecliptic_coordinate_system (wikipedia)
pub struct Spaceobj {
    position: (f64, f64, f64),   // longtitude (l), latitude (b), distance from sun (r) in AU
    mass: u64,                          // in kg, needed for accelerations
    lastupdate: time::PreciseTime,      // time for this position in the world
}

impl Spaceobj {
    pub fn new (longtitude: f64, latitude: f64, distance: f64, mass: u64) -> Spaceobj {
        Spaceobj {
            position: (longtitude, latitude, distance),
            mass: mass,
            lastupdate: time::PreciseTime::now(),
        }
    }
}
