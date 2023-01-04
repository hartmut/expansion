// Experimental Simulator of a cooperative solar system economy.
// implementation of Resource Storage

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use crate::core::production::parts::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect, Inspectable)]

enum ResourceType {
    Liquid, // in l
    Gas,    // TODO in l - at space ambient temperature. 
    Bulk,  // in kg, for use of raw materials like iron ore or iron itself
    Container, // in kg for assembled parts or premanufactured material
    None, // default
}

#[derive(Clone, Debug, PartialEq, Reflect, Inspectable)]

pub struct Resource {
    resource_type: ResourceType, // what type of is stored in this Store
    amount: u32,                 // in kg, l - if this is a container the value is 0
    mass: f32,                   // in kg, sum of all pices
    size: f32, //size of all piece in cm^3, if it is a liquid or gas one piece is always 1cm^3
    container: Vec<PartBundle>,   // vector with parts stored in this container, bundle defined in production - parts
}

impl Default for ResourceType {
    fn default() -> Self {
      ResourceType::None  
    }
    
}

impl Default for Resource {
    fn default() -> Self {
        Resource {
            resource_type: ResourceType::Bulk,
            amount: 0,
            mass: 0.0,
            size: 0.0,
            container: vec![],
        }
    }
}

impl Resource {

}
