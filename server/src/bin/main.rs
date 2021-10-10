// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

extern crate amethyst;
extern crate bevy;
extern crate expansion;

use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    utils::application_root_dir,
    Application,
};
use bevy::{prelude::*,log::LogPlugin};


use expansion::core::resource::ExpResources;
use expansion::core::resource;
use expansion::core::states::running_state::*;

// my mods to use
// use expansion::core::Core;

fn main() -> amethyst::Result<()> {
    // Bevy section
    let mut app = App::build();
    app.add_plugin(LogPlugin::default())
    .add_plugin(ExpResources);
    
   //TODO Load World

    // Amethyst section
    // set up assets directory
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");
    
    // create Dispatcher
    let mut dispatcher = expansion::core::system::new();
    dispatcher
    .add_bundle(resource::ExpResources)
    .add_bundle(TransformBundle);
    
    // build Application
    let game = Application::build(assets_dir, RunningState)?
    .with_frame_limit(FrameRateLimitStrategy::Sleep, 1)
    .build(dispatcher)?;
    
    // run world
    app.run();
    game.run();

    Ok(())
}
