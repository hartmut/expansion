// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use specs;

// solar cell energy production
// first step solar cells, needs to be generalized for other production methods
#[derive(Debug)]
pub struct SolarCell {
    production: f64, //Watt in optimal conditions https://en.wikipedia.org/wiki/Watt
}

impl specs::Component for SolarCell {
    type Storage = specs::VecStorage<Self>;
}

impl SolarCell {
    pub fn new() -> SolarCell {
        SolarCell { production: 10.0 }
    }
}
