// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

extern crate amethyst;
extern crate expansion;

use amethyst::{
    core::frame_limiter::FrameRateLimitStrategy, utils::application_root_dir, Application,
};

use expansion::core::resource;
use expansion::core::states::running_state::*;
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
    let _myconfig = config::Config::load_config(args);

    // set up assets directory
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");

    // create Dispatcher
    let mut dispatcher = expansion::core::system::new();
    dispatcher.add_bundle(resource::ExpResources);
    // TODO bundle loader component
    // TODO bundle loader system

    // build Application
    let game = Application::build(assets_dir, RunningState)?
        .with_frame_limit(FrameRateLimitStrategy::Sleep, 1)
        .build(dispatcher)?;

    //TODO Load World

    game.run();

    Ok(())
}
