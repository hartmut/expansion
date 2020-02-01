// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use specs;
// use specs::prelude::*;

//TODO enum which describes what object this is and unse the enum in the structure
//TODO rename the structure more appropriate
// Descriptions
#[derive(Debug)]
pub struct Desc {
    pub name: String,
    longname: String,
}

impl Desc {
    pub fn new(name: String, longname: String) -> Self {
        Desc { name, longname }
    }
}

impl specs::Component for Desc {
    type Storage = specs::VecStorage<Desc>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use specs::prelude::*;

    #[test]
    fn create_description_component() {
        let mut world = specs::World::new();
        world.register::<Desc>();
        world
            .create_entity()
            .with(Desc::new("Daniel Suarez".to_string(), "".to_string()))
            .build();
    }
}
