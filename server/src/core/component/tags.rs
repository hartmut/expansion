// Experimental Simulator of a cooperative solar system economy.
use bevy::prelude::*;

#[derive( Clone, Debug, PartialEq, Eq, Reflect, Default, Component)]
#[reflect(Component)]
#[allow(clippy::forget_non_drop)]
pub struct ModuleTag;

#[derive( Clone, Debug, PartialEq, Eq, Reflect, Default, Component)]
#[reflect(Component)]
#[allow(clippy::forget_non_drop)]
pub struct StationTag;


#[derive( Clone, Debug, PartialEq, Eq, Reflect, Default, Component)]
#[reflect(Component)]
#[allow(clippy::forget_non_drop)]
pub struct PlayerTag;