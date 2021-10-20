// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use bevy::prelude::*;

/*  in most cases inner and outer volume are the same. But if you have a container with a thick wall
/   or something similar the usabel volume will differ from the outer volume
*/

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct BasicParameter {
    pub mass: f64,   //in kg
    pub usablevol: f64, //in m^3
    pub outervol: f64, //in m^3
    pub extend: Vec3, //in m
}
