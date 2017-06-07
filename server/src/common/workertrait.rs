// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
/// all the traits in one file

use common::configuration::*;

/// worker trait and struct
#[derive(Debug)]
pub struct WorkerStruct {
    pub name: String, /* queue with incomming updates
                       * queue with outgoing updates */
}

pub trait WorkerTrait<T> {
    // propably we will need a btree map for the different storage files managed by a worker
    fn new(name: String, config: &Configuration) -> T;

    fn step(&mut self) {
        println!("one step", );
    }

    fn send_update(&self);

    fn get_update(&mut self);
}
