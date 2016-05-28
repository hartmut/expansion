// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
/// all the traits in one file

/// trait for Objects in this world
pub trait StdTrait<T> {
    fn update (&mut self);
    fn serialize (&self) -> String;
    fn new_from_deserialized (input: &String) -> T;
}

/// worker trait and struct

pub struct WorkerStruct {
    name: String,
    // queue with incomming updates
    // queue with outgoing updates
}

pub trait WorkerTrait {
    fn initalize (&mut self);
    fn send_update (&self);
    fn get_update (&mut self);
}
