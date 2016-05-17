// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// all the traits

// trait for Updates by Tick
pub trait WorldObject {
    fn update (&mut self);
    fn save (&self);
    fn load (&mut self);
}
