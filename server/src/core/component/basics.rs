// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

// first step solar cells, needs to be generalized for other production methods
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BasicParameter {
    mass: f64, //in kg
    volume: f64, //in m^3
               // further properties could be possible
}

impl specs::Component for BasicParameter {
    type Storage = specs::VecStorage<Self>;
}

impl BasicParameter {
    pub fn new_dummy() -> BasicParameter {
        BasicParameter {
            mass: 0.0,
            volume: 0.0,
        }
    }
}
