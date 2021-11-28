// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

/*  in most cases inner and outer volume are the same. But if you have a container with a thick wall
/   or something similar the usabel volume will differ from the outer volume. This is only the case 
/   for habitats, which has a value of habitable volume
/   if you need to integrate parts, they could be postioned in the habitable volume but also in the shell
/   between the habitat and the outer shell
*/

#[derive(Clone, Copy, Debug, PartialEq, Default, Reflect, Inspectable)]
pub struct BasicParameter {
    pub mass: f64,   //in kg
    pub volume: f64, //in m^3
    pub extend: Vec3, //in m
}
