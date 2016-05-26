// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// all the traits

// trait for Objects in this world
pub trait StdTrait<T> {
    fn update (&mut self);
    fn serialize (&self) -> String;
    fn new_from_deserialized (input: &String) -> T;
}
