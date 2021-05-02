// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use specs;
// use specs::prelude::*;

//TODO rename the structure more appropriate and change to legion

// Descriptions
#[derive(Debug)]
pub struct Desc {
    pub name: String,
    longtext: String,
}

impl Desc {
    pub fn new(name: String, longtext: String) -> Self {
        Desc { name, longtext }
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
