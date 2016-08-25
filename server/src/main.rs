// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

// macros and plugins
#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

// extern
extern crate time;
extern crate uuid;
extern crate serde_json;
extern crate toml;

// describe internal mods to use
mod physic;
mod structure;
mod common;
mod character;
mod tests;
mod components;

// my mods to use
use std::time::Duration;
use structure::station::AStation;
use structure::structure_worker::StructureWorker;
use character::player_worker::PlayerWorker;
use std::thread;
use common::stdtrait::StdTrait;
use common::workertrait::WorkerTrait;
use common::configuration;

// standard mods to use
use std::env;

// testincludes
// use tests::playertest;

fn main() {

    // move all arguments to a string vector
    let args: Vec<String> = env::args().collect();

    // panic if vector is too short
    if args.len() <= 1 { panic!("I need your input"); }

    // read configuration and data
    let mut tick_counter: u64 = 0;
    let myconfig = configuration::Configuration::load_config(args);

    // initalize timer and counter
    let tick_dur = Duration::from_secs(myconfig.get_tick());

    //create the player worker and initalize it
    let player_worker = PlayerWorker::new(  "Player_Worker".to_string(),
                                            myconfig.get_filenameplayer());

    //create the structure worker and initalize it
    let mut structure_worker = StructureWorker::new("Structure_Worker".to_string(),
                                                    myconfig.get_filenamestructure());
    structure_worker.initialize();

    // ticker input by webservice/json
    // TODO start webserver as an own thread to get informations from clients

    // wait, then update all objects,
    // wait for TICK Seconds in real time, this is analog to 2h in world time

    //tests
    let mut my_station = AStation::new("Firefly".to_string());
    loop {
        thread::sleep (tick_dur);
        tick_counter += 1;

        println!("Hello world, this is tick {}", tick_counter);
        println!("time elapsed since start: {} sec \n", tick_counter*myconfig.get_tick());

        //call the update methods of all relevant strutures for this tick
        my_station.update();
        println!("{} \n", my_station.serialize());

    }
}
