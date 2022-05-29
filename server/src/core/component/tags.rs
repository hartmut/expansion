// Experimental Simulator of a cooperative solar system economy.
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Inspectable, Clone, Debug, PartialEq, Eq, Reflect, Default, Component)]
#[reflect(Component)]
pub struct ModuleTag;

#[derive(Inspectable, Clone, Debug, PartialEq, Eq, Reflect, Default, Component)]
#[reflect(Component)]
pub struct StationTag;