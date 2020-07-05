// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

// use specs;
use specs::prelude::*;

mod desc;
pub use self::desc::Desc;

mod owner;
pub use self::owner::Owner;

mod account;
pub use self::account::Account;

mod habitat;
pub use self::habitat::Habitat;

pub mod location;
pub use self::location::Location;

pub mod partof;
pub use self::partof::PartOf;
pub mod basics;
pub use self::basics::BasicParameter;

pub mod energy;
pub use self::energy::battery::Battery;
pub use self::energy::solarcell::SolarCell;

// register all the components
pub fn new(world: &mut specs::World) {
    world.register::<Desc>();
    world.register::<Owner>();
    world.register::<Account>();
    world.register::<Habitat>();
    world.register::<Location>();
    world.register::<PartOf>();
    world.register::<Battery>();
    world.register::<SolarCell>();
}
