// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
/// basic component for asteroids, comets and small moons

/*
H(mag.)	    Absolute magnitude (asteroids only and set to "99.00" when unknown).
G 	  	    Magnitude slope parameter (asteroids only and set to "0.00" when H is unknown).
 Ref 	    Orbit solution reference.
*/

// enum - https://en.wikipedia.org/wiki/Asteroid_spectral_types
// TODO change name of this component to something more generic

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct SmallBody {
    // SOI - Orbital center (Earth, Sun, Jupiter, etc.) as enum
    /* center of world is sun, jupiter, earth ? default should be sun */
    jplno: u64, // JPL Numbering
    h: f64,     // (mag.) Absolute magnitude (asteroids only and set to "99.00" when unknown).
    g: f64,     // Magnitude slope parameter (asteroids only and set to "0.00" when H is unknown).
    osref: f64, // Orbit solution reference.
    diameter: f32, //in km
    extent: u32, //TODO represent in vector- in km tri(or bi)-axial body
    albedo: f32, // geometric albedo
    rotation: f32, //in h - body rotation period (synodic)
    gm: f32, // in km^3/s^2, standard gravitational parameter: product of the mass (M) and gravitational constant (G)
    bv: f32, // color index B-V magnitude difference
    ub: f32, // color index U-B magnitude difference
    ir: u32, //TODO represent in ir representation
    specb: u32, //TODO represent in enum type - SMASSII spectral taxonomic classification
    spect: u32, //TODO represent in enum type - Tholen spectral taxonomic classification
}
