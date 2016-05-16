// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// all the traits

// trait for Updates by Tick
pub trait TickUpdate {
    fn update (&mut self);
}

pub trait Storage {
    fn save (&self);
    fn load (&mut self);
}
