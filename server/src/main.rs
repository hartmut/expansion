// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

extern crate time;
extern crate uuid;
extern crate rustc_serialize;

// define external mods to use
use std::time::Duration;
use structure::station::AStation;
use std::thread;

// define internal mods to use
mod physic;
mod structure;
mod common;

// define constants
static TICK: u64 = 2; // one Tick is this much seconds long

fn main() {

    // read configuration

    // initalize timer and counter
    let tick_dur = Duration::from_secs(TICK);
    let mut tick_counter: u64 = 0;

    //tests
    let my_station =  AStation::new("Firefly".to_string());

    // ticker input by webservice/json
    // TODO start webserver as an own thread to get informations from clients

    // determine current world time from configuration
    // TODO let start = PreciseTime::now();

    // wait, then update all objects,
    // wait for 5 Minutes in real time, this is analog to 2h in world time
    loop {
        println!("Hello world, this is tick {}", tick_counter);
        println!("time elapsed since start is {} sec \n", tick_counter*TICK);

        thread::sleep (tick_dur);
        tick_counter += 1;
        //TODO call the update methods of all relevant strutures for this tick
    }


}
