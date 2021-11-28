// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
/// basic component for asteroids, comets and small moons
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

// spectral enums - https://en.wikipedia.org/wiki/Asteroid_spectral_types

#[derive(Clone, Copy, Debug, PartialEq, Reflect, Inspectable)]
pub enum SpecT {
    None,
    A, // https://en.wikipedia.org/wiki/A-type_asteroid
    C,
    B,
    D,
    E,
    F,
    G,
    M,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
}

impl Default for SpecT {
    fn default() -> Self {SpecT::None}
}


#[derive(Clone, Copy, Debug, PartialEq, Reflect, Inspectable)]
pub enum SpecB {
    None,
    A,
    B,
    C,
    Cb,
    Ch,
    Cg,
    Chg,
    D,
    X,
    Xc,
    Xe,
    Xk,
    Q,
    R,
    S,
    Sa,
    Sk,
    Sl,
    Sq,
    Sr,
    T,
    V,
    K,
    L,
    Ld,
    O,
}

impl Default for SpecB {
    fn default() -> Self {SpecB::None}
}

/*
H(mag.)	    Absolute magnitude (asteroids only and set to "99.00" when unknown).
G 	  	    Magnitude slope parameter (asteroids only and set to "0.00" when H is unknown).
 Ref 	    Orbit solution reference.
*/

#[derive(Clone, Copy, Debug, PartialEq, Reflect, Inspectable, Default)]
pub struct AsteroidProp {
    // SOI - Orbital center (Earth, Sun, Jupiter, etc.) as enum
    /* center of world is sun, jupiter, earth ? default should be sun */
    jplno: u64,           // JPL Numbering
    h: f64,     // (mag.) Absolute magnitude (asteroids only and set to "99.00" when unknown).
    g: f64,     // Magnitude slope parameter (asteroids only and set to "0.00" when H is unknown).
    osref: f64, // Orbit solution reference.
    diameter: f32, //in km
    extent: Vec3, //in km tri(or bi)-axial body
    albedo: f32, // geometric albedo
    rotation: f32, //in h - body rotation period (synodic)
    gm: f32, // in km^3/s^2, standard gravitational parameter: product of the mass (M) and gravitational constant (G)
    bv: f32, // color index B-V magnitude difference
    ub: f32, // color index U-B magnitude difference
    ir: f32, // color index I-R magnitude difference
    specb: SpecB, //SMASSII spectral taxonomic classification
    spect: SpecT, //Tholen spectral taxonomic classification
}
