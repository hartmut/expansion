// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

// solar cell energy production
// first step solar cells, needs to be generalized for other production methods
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SolarCell {
    production: f64, //Watt in optimal conditions https://en.wikipedia.org/wiki/Watt
                     // parameters like conditions
}

impl SolarCell {
    pub fn default() -> SolarCell {
        SolarCell { production: 10.0 }
    }
}
