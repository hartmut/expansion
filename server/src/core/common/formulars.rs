use bevy::prelude::*;

/// gets an extension (size of object) and a mass per sqm
/// outputs the complete mass of the object
pub fn mass_sqm(ext: Vec3, mass: f32) -> f32 {
    (ext.x * ext.y * 2.0 + ext.x * ext.z * 2.0 + ext.y * ext.z * 2.0) * mass
}

/// volume of Vec3
pub fn volume(ext:Vec3) -> f32 {
    ext.x * ext.y * ext.z
}