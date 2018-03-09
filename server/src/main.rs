// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

// macros and plugins
#![allow(dead_code)]
#![warn(unused_variables)]
// #![warn(unused_mut)]
// #![feature(proc_macro)]
#[macro_use]

// extern
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate time;
extern crate uuid;
extern crate toml;
extern crate chrono;
extern crate rand;
extern crate specs;

// describe internal mods to use
mod physic;
mod structure;
mod common;
mod character;
mod tests;
mod recipes;
mod core;

// standard mods to use
use std::env;
// use std::thread;
// use std::time::Duration;

// use common::configuration;
use common::configuration;

// my mods to use
use structure::structure_worker::StructureWorker;
use character::player_worker::PlayerWorker;
use common::workertrait::WorkerTrait;
use core::Core;


// testincludes
// use tests::playertest;
// TODO replace all unwraps with proper error handling

fn main() {

    // move all arguments to a string vector
    let args: Vec<String> = env::args().collect();

    // panic if vector is too short
    if args.len() <= 1 {
        panic!("I need a config file");
    }

    // read configuration and data
    let myconfig = configuration::Configuration::load_config(args);

    // create the player worker and initalize it
    let _player_worker = PlayerWorker::new("Player_Worker".to_string(), &myconfig);
    // create the structure worker and initalize it
    let _structure_worker = StructureWorker::new("Structure_Worker".to_string(), &myconfig);

    // create the core
    let mut core = Core::new(&myconfig);

    // ticker input by webservice/json
    // TODO start webserver as an own thread to get informations from clients

    // core loop, all the management is done in the systems and core.step()
    loop {
        core.step();
    }
}
