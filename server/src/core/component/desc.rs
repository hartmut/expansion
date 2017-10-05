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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_description_component() {
        let mut world = specs::World::new();
        world.register::<Desc>();
        world.create_entity()
            .with(Desc::new("Daniel Suarez".to_string(), "".to_string()));
    }
}
