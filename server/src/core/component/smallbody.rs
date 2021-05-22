// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
/// basic component for asteroids, comets and small moons

/// Astronomical Database: https://ssd.jpl.nasa.gov/?sb_elem ascending
/// and https://www.asterank.com/

/*
H(mag.)	    Absolute magnitude (asteroids only and set to "99.00" when unknown).
G 	  	    Magnitude slope parameter (asteroids only and set to "0.00" when H is unknown).
 Ref 	    Orbit solution reference.
*/

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Orbital {
    // SOI - Orbital center (Earth, Sun, Jupiter, etc.) as enum
    /* center of world is sun, jupiter, earth ? default should be sun */
    jplno: u64, // JPL Numbering
    h: f64,     // (mag.) Absolute magnitude (asteroids only and set to "99.00" when unknown).
    g: f64,     // Magnitude slope parameter (asteroids only and set to "0.00" when H is unknown).
    osref: f64, // Orbit solution reference.
}
