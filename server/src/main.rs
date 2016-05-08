// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

extern crate time;

mod physic;

use physic::location::Spaceobj;
use std::time::Duration;
use std::thread;

fn main() {

    // define statics
    static TICK: u64 = 2; // one tick is this much seconds long

    // read configuration

    // initalize timer and counter
    let tick_dur = Duration::from_secs(TICK);
    let mut tick_counter: u64 = 0;

    let x = Spaceobj::new(1.0, 12.0 ,3.0 ,4 ,tick_counter);

    // ticker input by webservice/json
    // TODO start webserver as an own thread to get informations from clients

    // define start time from configuration
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
