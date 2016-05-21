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

// describe external mods to use
use std::time::Duration;
use structure::station::AStation;
use std::thread;
use common::traits::StdTrait;

// define constants
static TICK: u64 = 2; // one Tick is this much seconds long and 2h worth in the world

fn main() {

    // read configuration

    // initalize timer and counter
    let tick_dur = Duration::from_secs(TICK);
    let mut tick_counter: u64 = 0;

    //tests
    let mut my_station = AStation::new("Firefly".to_string());
    let serialized = my_station.serialize();
    println!("{} \n", serialized);
    let alternativetempstation: AStation = serde_json::from_str(&serialized).unwrap(); // whats easier - this
    let mut tempstation: AStation = AStation::new("a further Firefly".to_string()); // or this?
    tempstation = AStation::deserialize(&tempstation, &serialized);
    println!("{:?} \n", tempstation);

    // ticker input by webservice/json
    // TODO start webserver as an own thread to get informations from clients

    // determine current world time from configuration
    // TODO let start = PreciseTime::now();

    // wait, then update all objects,
    // wait for 5 Minutes in real time, this is analog to 2h in world time
    loop {
        thread::sleep (tick_dur);
        tick_counter += 1;

        println!("Hello world, this is tick {}", tick_counter);
        println!("time elapsed since start: {} sec \n", tick_counter*TICK);

        //TODO call the update methods of all relevant strutures for this tick
        my_station.update();
        println!("{:?} \n", my_station);

    }
}
