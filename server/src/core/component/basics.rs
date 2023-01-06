// Experimental Simulator of a cooperative solar system economy.
// See doc/LICENSE for licensing information

use crate::core::common::formulars::*;
use bevy::prelude::*;

/*  in most cases inner and outer volume are the same. But if you have a container with a thick wall
/   or something similar the usabel volume will differ from the outer volume. This is only the case
/   for habitats, which has a value of habitable volume
/   if you need to integrate parts, they could be postioned in the habitable volume but also in the shell
/   between the habitat and the outer shell
*/

// TODO how to define the extend of the station? needs 3d grid first to define this box
#[derive(Clone, Copy, Debug, PartialEq, Default, Reflect, Component)]
#[reflect(Component)]
pub struct BasicParameter {
    mass: f32,    //in kg
    volume: f32,  //in m^3
    extend: Vec3, //in m
}

impl BasicParameter {
    pub fn new(ext: Vec3, mass: f32) -> BasicParameter {
        BasicParameter {
            mass,
            volume: volume(ext),
            extend: ext,
        }
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }
}
