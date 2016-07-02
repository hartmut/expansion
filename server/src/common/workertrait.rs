// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
/// all the traits in one file

/// worker trait and struct

#[derive(Debug)]
pub struct WorkerStruct {
    pub name: String,
    // queue with incomming updates
    // queue with outgoing updates
}

pub trait WorkerTrait<T> {
    fn new (name: String) -> T;
    fn initalize (&mut self, filename: String);
    fn send_update (&self);
    fn get_update (&mut self);
    fn run(&mut self);
}
