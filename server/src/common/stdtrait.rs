// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
/// all the traits in one file

/// trait for Objects in this world
use super::myuuid::*;

pub trait StdTrait<T> {
    fn new_from_deserialized(input: &String) -> T;

    fn step(&mut self) {
        println!("one step", );
    }

    fn getuuid(&self) -> ExpUuid;

    fn serialize(&self) -> String;
}
