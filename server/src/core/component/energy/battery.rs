// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

// energy store and production
// first step solar cells, needs to be generalized for other production methods
// NOTE assumption how much volt?
#[derive(Clone, Debug, PartialEq)]
pub struct Battery {
    act_storage: f64, //Ah https://en.wikipedia.org/wiki/Ampere_hour
    max_storage: f64, //Ah https://en.wikipedia.org/wiki/Ampere_hour
}

impl Battery {
    pub fn new() -> Battery {
        Battery {
            act_storage: 0.0,
            max_storage: 0.0,
        }
    }
}
