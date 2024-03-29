/// calculations of positions in solar system
/// pos based on Ecliptic_coordinate_system (wikipedia)
/// Orbital elements https://en.wikipedia.org/wiki/Orbital_elements
use bevy::prelude::*;

/*
Epoch(MJD)  Epoch of the elements represented as the Modified Julian Date (MJD),
which is defined as the Julian date - 2400000.5. - https://crates.io/crates/hifitime
a(AU) 	    Semimajor axis of the orbit (asteroids only).
q(AU) 	    Perihelion distance (comets only).
e 	  	    Eccentricity of the orbit.
i(deg.)     Inclination of the orbit with respect to the J2000 ecliptic plane.
w(deg.)     Argument of perihelion (J2000-Ecliptic).
Node(deg.)  Longitude of the ascending node (J2000-Ecliptic).
M(deg.)	    Mean anomaly at epoch (asteroids only).
Tp 	  	    Time of perihelion passage (comets only), formatted as a calendar date
(YYYYMMDD.DDD) where "YYYY" is the year, "MM" is the numeric month,
and "DD.DDD" is the day and day fraction.
*/
// COMBAK work on orbital informations
#[derive(Clone, Copy, Debug, PartialEq, Reflect, Default, Component)]
#[reflect(Component)]
pub struct Orbital {
    // SOI - Orbital center (Earth, Sun, Jupiter, etc.) as enum
    /* center of world is sun, jupiter, earth ? default should be sun */
    epoch: f64, /* TODO (MJD) Epoch of the elements represented as the Modified Julian Date (MJD),
                which is defined as the Julian date - 2400000.5. - https://crates.io/crates/hifitime */
    a: f64,    // (AU) Semimajor axis of the orbit (asteroids only)
    q: f64,    // (AU) Perihelion distance (comets only).
    e: f64,    // eccentricity of orbit
    i: f64,    // (deg.) Inclination of the orbit with respect to the J2000 ecliptic plane.
    w: f64,    // (deg.) Argument of perihelion (J2000-Ecliptic).
    node: f64, // (deg.) Longitude of the ascending node (J2000-Ecliptic).
    m: f64,    // (deg.) Mean anomaly at epoch (asteroids only).
    tp: f64,   /* TODO Time of perihelion passage (comets only), formatted as a calendar date
               (YYYYMMDD.DDD) where "YYYY" is the year, "MM" is the numeric month,
               and "DD.DDD" is the day and day fraction.*/
}
