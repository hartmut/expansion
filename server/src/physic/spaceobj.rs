// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// calculations of positions in solar system based on keplers law

extern crate time;

// pos based on Ecliptic_coordinate_system (wikipedia)
struct Spaceobj {
    position: (f64, f64, f64),   // longtitude (l), latitude (b), distanz from sun (r) in AU
    mass: u64,                          // in kg, needed for accelerations
    lastupdate: time::PreciseTime,      // time for this position in the world
}

pub fn spaceobj_new () {

    println!("hello world")
}
