// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// central worker, using for ticks in the "universe"


use time::{Duration, PreciseTime};

let start = PreciseTime::now();

while start.to(PreciseTime::now()) < Duration::seconds(1) {
    //call the different update methods for the next tick
}
