// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

extern crate time;

mod physic;

use physic::location::Spaceobj;
use time::{Duration, PreciseTime};
use std::time::Duration;

fn main() {

    // read configuration

    // initalize
    let x = Spaceobj::new(1.0, 2.0, 3.0, 4);

    // ticker input by webservice/json
    // start webserver as an own thread to get informations from clients

    // define start time from configuration
    // let start = PreciseTime::now();

    // wait, then update all objects,
    // wait for 5 Minutes in real time, this is analog to 2h in world time
    loop {
        sleep (from_secs(300))
        //call the different update methods for the next tick
    }


}
