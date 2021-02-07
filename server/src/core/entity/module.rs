use core::component::*;
use specs;
use specs::prelude::*;

pub fn new(world: &mut specs::World, name: String, _parent: Entity) -> Entity {
    world
        .create_entity()
        .with(Desc::new(name, "".to_string()))
        .build()
}
