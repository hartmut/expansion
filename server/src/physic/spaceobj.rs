// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing informati
//
// calculations of positions in solar system based on keplers law

extern crate time;

struct Spaceobj {
    position: (f32, f32, u64),   // degree1, deegree2, distanz from sun
    mass: u64,                   // in kg, needed for accelerations
    lastupdate: time::PreciseTime,     // time for this position in the world
}

pub fn spaceobj_new () {
    
    println!("hello world")
}
