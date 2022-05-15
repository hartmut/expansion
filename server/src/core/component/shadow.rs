// Experimental Simulator of a cooperative solar system economy.
// implementation of shadow data for summing of componentes over stations and modules

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use crate::core::component::*;

/*  shadow structure for updating components which are needed to get propagated
/ upwards in the station/module/component hirarchy
*/

// COMEBACK write system for updating shadow in the station hirarchy
#[derive(Clone, Copy, Debug, PartialEq, Default, Reflect, Inspectable, Component)]
#[reflect(Component)]
pub struct Shadow {
    pub energy: energy::Energy,
}
