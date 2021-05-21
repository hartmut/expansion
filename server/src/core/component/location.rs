// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// calculations of positions in solar system

// pos based on Ecliptic_coordinate_system (wikipedia)

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Location {
    long: f64, // longtitude (l) in relation to sun
    lat: f64,  // latitude (b) in relation to sun
    dist: f64, // distance from sun (r) in AU
                /* center of world is sun, jupiter, earth ? default should be sun */
}

impl Location {
    pub fn new(longtitude: f64, latitude: f64, distance: f64) -> Location {
        Location {
            long: longtitude,
            lat: latitude,
            dist: distance,
        }
    }
}
