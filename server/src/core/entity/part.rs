use crate::core::component::*;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Bundle, Reflect, Inspectable, Component)]
pub struct Part {
    desc: desc::Desc,
    basics: basics::BasicParameter,
    energy: energy::Energy,
}