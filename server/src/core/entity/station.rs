// Copyright (C) 2016  Hartmut Prochaska
// Experimental Simulator of a cooperative solar system economy.
// See doc/LICENSE for licensing information
use amethyst::{
    core::{transform::Parent, Transform},
    prelude::*,
};
use core::component::desc::Desc;

pub struct Station;

/// a station has the following parts
/// - a description
/// - an owner as a parent
/// - modules as children which are arranged in a matrix

impl Station {
    pub fn new(world: &mut World, name: String, owner: Entity) -> Entity {
        let desc = Desc::new(name, "".to_string());
        let owner = Parent(owner);
        let transform = Transform::default();
        // TODO insert this station into the child list of the player
        world.push((desc, owner,transform))
    }
}
