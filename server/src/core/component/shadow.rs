// Experimental Simulator of a cooperative solar system economy.
// implementation of shadow data for summing of componentes over stations and modules

use crate::core::component::*;
use bevy::prelude::*;

/*  shadow structure for updating components which are needed to get propagated
/ upwards in the station/module/component hirarchy
*/

// TODO write system for updating shadow in the station hirarchy
#[derive(Clone, Copy, Debug, PartialEq, Default, Reflect, Component)]
#[reflect(Component)]
pub struct Shadow {
    pub energy: energy::Energy,
}

impl Shadow {
    pub fn add_energy(&mut self, e: &energy::Energy) {
        self.energy.act_storage += e.act_storage;
        self.energy.max_storage += e.max_storage;
        self.energy.production += e.production;
        self.energy.consumption += e.consumption;
    }
}
