// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// descriptions for entities
use specs;

pub struct Desc {
    pub name: String,
    pub longname: String,
}

impl Desc {
    pub fn new(name: String, longname: String) -> Self {
        Desc {
            name: name,
            longname: longname,
        }
    }
}

impl specs::Component for Desc {
    type Storage = specs::VecStorage<Desc>;
}
