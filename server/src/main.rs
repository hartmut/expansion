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

// describe internal mods to use
mod physic;
mod structure;
mod common;
mod character;
mod tests;

// describe external mods to use
use std::time::Duration;
use structure::station::AStation;
use std::thread;
use common::traits::StdTrait;

// testincludes
//use tests::playertest;

// define constants
const TICK: u64 = 2; // one Tick is this much seconds long and 2h worth in the world
static mut tick_counter: u64 = 0;

fn main() {

    // read configuration and data

    // initalize timer and counter
    let tick_dur = Duration::from_secs(TICK);

    //tests
    let mut my_station = AStation::new("Firefly".to_string());

    // ticker input by webservice/json
    // TODO start webserver as an own thread to get informations from clients

    // determine current world time from configuration
    // TODO let start = PreciseTime::now();

    // wait, then update all objects,
    // wait for TICK Seconds in real time, this is analog to 2h in world time
    loop {
        thread::sleep (tick_dur);
        unsafe {
            tick_counter += 1;

            println!("Hello world, this is tick {}", tick_counter);
            println!("time elapsed since start: {} sec \n", tick_counter*TICK);
        }

        //TODO call the update methods of all relevant strutures for this tick
        my_station.update();
        println!("{:?} \n", my_station);

    }
}
