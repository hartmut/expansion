// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use specs;

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
    use specs::world::Builder;

    #[test]
    fn create_description_component() {
        let mut world = specs::World::new();
        world.register::<Desc>();
        world
            .create_entity()
            .with(Desc::new("Daniel Suarez".to_string(), "".to_string()));
    }
}
