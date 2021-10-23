// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

// mod debug_system;
pub mod update_worldtime;
use bevy::prelude::*;

pub struct ExpSystems;

impl Plugin for ExpSystems {
    fn build(&self, app: &mut AppBuilder) {

        // insert systems for step updates
        app.add_system_to_stage(
            CoreStage::Update,
            update_worldtime::update_worldtime.system(),
        ); // TODO configure delay between steps
    }
}