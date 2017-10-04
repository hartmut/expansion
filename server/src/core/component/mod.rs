// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
use specs;

mod desc;
pub use self::desc::Desc;

pub fn new(world: &mut specs::World) {
    world.register::<Desc>();
}
