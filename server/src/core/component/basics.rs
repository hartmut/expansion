// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

// first step solar cells, needs to be generalized for other production methods
#[derive(Clone, Copy, Debug, PartialEq,Default)]
pub struct BasicParameter {
    pub mass: f64, //in kg
    pub volume: f64, //in m^3
}
