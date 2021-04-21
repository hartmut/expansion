// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

extern crate amethyst;
extern crate expansion;

use amethyst::{
    prelude::*,
    utils::application_root_dir,
    GameDataBuilder,
    Application};

use expansion::utils::config;
use expansion::core::states::running_state::*;
use std::env;

// my mods to use
use expansion::core::Core;

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

    // build GameData
    let game_data = GameDataBuilder::default();

    // build Application
    let mut game = Application::new(assets_dir, RunningState, game_data)?;

    // create the core
    let mut core = Core::new(&myconfig);

    // change to amethyst loop management
    // core loop, all the management is done in the systems and core.step()
    loop {
        core.step();
    }
}
