// Experimental Simulator of a cooperative solar system economy.
// Energy component usable for storage, production and consumption in modules, parts and also the station (there you will see summed up data)

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Clone, Copy, Debug, PartialEq, Reflect, Inspectable, Default, Component)]
#[reflect(Component)]

// TODO change energy variables to f32
pub struct Energy {
    pub act_storage: f64, //Wh currently in this entity, https://en.wikipedia.org/wiki/Grid_energy_storage#Batteries
    pub max_storage: f64, //Wh maximal available in this entity, https://en.wikipedia.org/wiki/Grid_energy_storage#Batteries
    pub production: f64,  // in Watt    
    pub consumption: f64, // in Watt
}

impl Energy {
    pub fn set (&mut self, e: Energy) {
        self.act_storage = e.act_storage;
        self.max_storage = e.max_storage;
        self.production = e.production;
        self.consumption = e.consumption;
    }
}