// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
use specs;

pub mod desc;
pub use self::desc::Desc;

mod owner;
pub use self::owner::Owner;

mod account;
pub use self::account::Account;

mod partof;
pub use self::partof::Partof;

mod o2;
pub use self::o2::O2;

// register all the components
pub fn new(world: &mut specs::World) {
    world.register::<Desc>();
    world.register::<Owner>();
    world.register::<Account>();
    world.register::<Partof>();
    world.register::<O2>();
}
