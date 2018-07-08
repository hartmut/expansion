// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
use specs;
use specs::world::Builder;
use core::component::*;

pub fn new(world: &mut specs::World, name: String, owner: specs::world::Index) -> specs::world::Index {
    let station: specs::Entity = world
        .create_entity()
        .with(Owner::new(owner))
        .with(Desc::new(name, "".to_string()))
        .with(HasParts::new())
        .with(O2::new())
        .build();
    station.id()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn newworld() -> specs::World {
        let mut world = specs::World::new();
        world.register::<Owner>();
        world.register::<Desc>();
        world.register::<HasParts>();
        world.register::<O2>();
        world
    }

    #[test]
    fn compare_station_ids() {
        let mut world = newworld();
        let station1: specs::world::Index = new(&mut world, "ISS".to_string(), 1);
        let station2: specs::world::Index = new(&mut world, "Moon".to_string(), 2);
        assert_ne!(station1, station2);
    }

}
