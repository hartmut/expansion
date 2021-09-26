// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use amethyst::core::math::Vector3;

// first step solar cells, needs to be generalized for other production methods
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BasicParameter {
    pub mass: f64,   //in kg
    pub volume: f64, //in m^3
    pub extend: Vector3<f32>, //in m
}
