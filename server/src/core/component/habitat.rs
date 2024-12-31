// Experimental Simulator of a cooperative solar system economy.
// See doc/LICENSE for licensing information

use crate::core::common::formulars::*;
use crate::core::entity::*;
use bevy::prelude::*;
use measurements::pressure::*;

/// when a module has an athmosphere this component is used
/// is part of a module, volume must be smaller than the whole module
/// people in the habitat will be managed in the component people
#[derive(Clone, Copy, Debug, PartialEq, Default, Reflect, Component)]
#[reflect(Component)]
pub struct Habitat {
    // in m^3
    // TODO rewrite so that volume is used from basic component by using a system for update of athomsphere
    volume: f32,

    // Atmosphere
    // in kg, from this values and the volume you get the partial pressure
    o2: f64,
    n2: f64,
    co2: f64,

    // pressure in atm, falls if you don't have enough consumables
    // calculate pressure from volume, temperature and kg of gases
    k_pa: f64, // calculated

    // futher environment variables
    temperature: f64,
}

impl Habitat {
    // start with a human habitable athmosphere
    pub fn new(volume: f32) -> Self {
        Habitat {
            volume,
            // based on calculator https://www.engineeringtoolbox.com/oxygen-O2-density-specific-weight-temperature-pressure-d_2082.html
            // gases are in kg in the available volume of the habitat, funnitures are not considered
            o2: 1.314 * volume as f64 * 0.2,
            // based on calculator https://www.engineeringtoolbox.com/nitrogen-N2-density-specific-weight-temperature-pressure-d_2039.html
            n2: 1.16 * volume as f64 * 0.8,
            co2: 0.0,
            // source https://spaceflight.nasa.gov/shuttle/reference/shutref/orbiter/eclss/cabinpress.html - one atm or this kPa
            k_pa: 101.325,
            temperature: 20.0,
        }
    }

    // source https://en.wikipedia.org/wiki/Partial_pressure#In_diving_breathing_gases
    // TODO find pressure and calc library and correct the formula
    // TODO rewrite as part of the component
    fn o2_part_pressure(&self) -> f64 {
        (self.o2 / self.volume as f64) * Pressure::from_kilopascals(self.k_pa).as_atmospheres()
    }

    // not enough partial pressure with oxygen
    fn hypoxia(&self) -> bool {
        // The minimum safe lower limit for the partial pressures of oxygen in a gas mixture is 0.16 bars
        0.16 > self.o2_part_pressure()
    }

    /// before adding the habitat you need to check whether it will fit into the module
    /// if mass is 0.0 then we assume that the pressurized part needs 50kg/sqm of material
    pub fn add_part_habitat<'w, 's>(
        mut commands: Commands<'w, 's>,
        parent: Entity,
        ext: Vec3,
        mass: f32,
    ) -> Commands<'w, 's> {
        let mut mass = mass;
        if mass == 0.0 {
            // 50kg/sqm, assumtption this is a quader
            mass = mass_sqm(ext, 50.0);
        }
        let habitat_part = commands
            .spawn(part::Part::create("Habitat", ext, mass))
            .id();
        let habitat = Habitat::new(6.0);
        // #TODO only Bundle can be connected with child/parent connections
        // commands.entity(habitat_part).insert(habitat);
        // commands.entity(parent).with_child(&[habitat_part]);
        commands
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hypoxia_false() {
        let habitat = Habitat {
            volume: 1.,
            o2: 1.1314 * 1.0 * 0.2,
            n2: 1.16 * 1.0 * 0.8,
            co2: 0.0,
            k_pa: 101.325,
            temperature: 20.0,
        };
        let o2_part_pressure = habitat.o2_part_pressure();
        assert_eq!(o2_part_pressure, 0.22628);
        let hypoxia = habitat.hypoxia();
        assert!(!hypoxia);
    }

    #[test]
    fn test_hypoxia_true() {
        let habitat = Habitat {
            volume: 1.,
            o2: 1.1314 * 1.0 * 0.10,
            n2: 1.16 * 1.0 * 0.8,
            co2: 0.0,
            k_pa: 101.325,
            temperature: 20.0,
        };
        let hypoxia = habitat.hypoxia();
        assert!(hypoxia);
    }
}
