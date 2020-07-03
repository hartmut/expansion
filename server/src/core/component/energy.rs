// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use specs;

// energy store and production
// first step solar cells, needs to be generalized for other production methods
#[derive(Debug)]
pub struct Energy {
    act_storage: f64, //Ah https://en.wikipedia.org/wiki/Ampere_hour
    max_storage: f64, //Ah https://en.wikipedia.org/wiki/Ampere_hour
    production: f64,  //Watt in optimal conditions https://en.wikipedia.org/wiki/Watt
}

impl specs::Component for Energy {
    type Storage = specs::VecStorage<Self>;
}

impl Energy {
    pub fn new() -> Energy {
        Energy {
            act_storage: 0.0,
            max_storage: 0.0,
            production: 10.0,
        }
    }
}
