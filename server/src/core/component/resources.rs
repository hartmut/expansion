// Experimental Simulator of a cooperative solar system economy.
// implementation of Resource Storage

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect, Inspectable)]
enum ResourceType {
    Liquid, // in l
    Gas,    // in l
    Solid,  // in kg
}

#[derive(Clone, Copy, Debug, PartialEq, Reflect, Inspectable)]
pub struct Resource {
    resource_type: ResourceType, // what type of is stored in this Store
    pieces: u32,                 // in l or pieces
    mass: f64,                   // in kg, sum of all pices
    size: f64, //size of all piece in cm^3, if it is a liquid or gas one piece is always 1cm^3
}

impl Default for Resource {
    fn default() -> Resource {
        Resource {
            resource_type: ResourceType::Solid,
            pieces: 0,
            mass: 0.0,
            size: 0.0,
        }
    }
}

impl Resource {
    fn get_mass_per_piece(self) -> f64 {
        self.mass / self.pieces as f64
    }

    fn get_size_per_piece(self) -> f64 {
        self.size / self.pieces as f64
    }
}
