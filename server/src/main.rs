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

// my mods to use
use std::time::Duration;
use structure::station::AStation;
use character::player::Player;
use character::player_worker::PlayerWorker;
use std::thread;
use common::traits::StdTrait;
use common::configuration;

// standard mods to use
use std::env;

// testincludes
// use tests::playertest;

fn main() {
    // read inputdata
    let args: Vec<String> = env::args().collect();

    // read configuration and data
    let mut tick_counter: u64 = 0;
    let myconfig = configuration::Configuration::load_config(args);

    // initalize timer and counter
    let tick_dur = Duration::from_secs(myconfig.get_tick());

    //tests
    let mut my_station = AStation::new("Firefly".to_string());
    let mut one_player = Player::new("Ian Banks".to_string());
    let mut player_worker = PlayerWorker::new("Player_Worker".to_string());

    println!("{:?}", one_player);
    // ticker input by webservice/json
    // TODO start webserver as an own thread to get informations from clients

    // wait, then update all objects,
    // wait for TICK Seconds in real time, this is analog to 2h in world time
    loop {
        thread::sleep (tick_dur);
        tick_counter += 1;

        println!("Hello world, this is tick {}", tick_counter);
        println!("time elapsed since start: {} sec \n", tick_counter*myconfig.get_tick());

        //call the update methods of all relevant strutures for this tick
        my_station.update();
        let serialized = my_station.serialize();
        println!("{} \n", serialized);
        //println!("{:?} \n", my_station);

    }
}
