// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

extern crate amethyst;
extern crate expansion;

use amethyst::{utils::application_root_dir, Application};

use expansion::core::states::running_state::*;
use expansion::core::resource::*;
use expansion::utils::config;

use std::env;

// my mods to use
// use expansion::core::Core;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    // move all arguments to a string vector
    let args: Vec<String> = env::args().collect();
    // panic if vector is too short
    if args.len() <= 1 {
        panic!("I need a config file");
    }

    // read configuration and resources
    let myconfig = config::Config::load_config(args);

    // set up assets directory
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");

    let dispatcher = expansion::core::system::new();

    // build Resources
    let worldtime = Worldtime::new(myconfig.get_tick(), myconfig.get_tick_length());

    // build Application
    let game = Application::build(assets_dir, RunningState)?
        .with_resource(worldtime)
        .build(dispatcher)?;

    game.run();

    Ok(())
}
