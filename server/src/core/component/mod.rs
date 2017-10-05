// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
use specs;

mod desc;
pub use self::desc::Desc;

mod player;
pub use self::player::Player;

mod account;
pub use self::account::Account;

pub fn new(world: &mut specs::World) {
    world.register::<Desc>();
    world.register::<Player>();
    world.register::<Account>();
}
