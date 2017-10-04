// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

// macros and plugins
#![allow(dead_code)]
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

// my mods to use
use structure::structure_worker::StructureWorker;
use character::player_worker::PlayerWorker;
use common::workertrait::WorkerTrait;
use core::Core;

// use common::configuration;
use common::configuration;

// standard mods to use
use std::env;
use std::thread;
use std::time::Duration;

// testincludes
// use tests::playertest;
// TODO replace all unwraps with proper error handling

fn main() {

    // move all arguments to a string vector
    let args: Vec<String> = env::args().collect();

    // panic if vector is too short
    if args.len() <= 1 {
        panic!("I need your input");
    }

    // read configuration and data
    let myconfig = configuration::Configuration::load_config(args);

    // initalize timer and counter
    let mut tick_counter: u64 = 1;
    let tick_dur = Duration::from_secs(myconfig.get_tick());
    let tick_length = time::Duration::hours(myconfig.get_tick_length());
    let mut worldtime = chrono::DateTime::parse_from_rfc2822("1 Jan 2023 00:00:00 +0000").unwrap();

    // create the player worker and initalize it
    let mut player_worker = PlayerWorker::new("Player_Worker".to_string(), &myconfig);
    // create the structure worker and initalize it
    let mut structure_worker = StructureWorker::new("Structure_Worker".to_string(), &myconfig);

    // ticker input by webservice/json
    // TODO start webserver as an own thread to get informations from clients

    // wait, then update all objects,
    // wait for TICK Seconds in real time, this is analog to xh in world time, look at the config
    // loop {
    //
    //     println!("\nHello world, this is tick {}", tick_counter);
    //     println!("time elapsed since start: {} sec \n",
    //              tick_counter * myconfig.get_tick());
    //     println!("the current worldtime is {:?}", worldtime);
    //
    //     // call the update methods of all relevant strutures for this tick
    //     structure_worker.step();
    //     player_worker.step();
    //
    //     // time management
    //     tick_counter += 1;
    //     thread::sleep(tick_dur);
    //     worldtime = worldtime + tick_length;
    // }

    // initalize Core
    let mut core = Core::new();

    // loop Core
    loop {
        core.step();
    }
}
