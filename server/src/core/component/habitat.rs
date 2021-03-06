// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use measurements::pressure::*;

pub struct ChemMix {
    molecule: String,
    kg: f64,
}

// when a module has an athmospere this component is used
/// is part of a module, volume must be smaller than the whole module
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Habitat {
    // in m^3
    volume: u64, // NOTE create module basics

    // Athmosphere
    // TODO redesign and split this component
    // in kg, from this values and the volume you get the partial pressure
    // TODO array of gases, could also be used for other purposes probably
    o2: f64,
    n2: f64,
    co2: f64,
    // pressure in atm, falls if you don't have enough consumables
    // calculate pressure from volume, temperature and kg of gases
    k_pa: f64, // calculated

    // futher environment variables
    temperature: f64,

    // how many people are currently in the habitat
    person_count: u32,
}

impl Habitat {
    // start with a human habitable athmosphere
    pub fn new(volume: u64) -> Self {
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
            person_count: 0,
        }
    }

    // source https://en.wikipedia.org/wiki/Partial_pressure#In_diving_breathing_gases
    // TODO find pressure and calc library and correct the formula
    fn o2_part_pressure(&self) -> f64 {
        (self.o2 / self.volume as f64) * Pressure::from_kilopascals(self.k_pa).as_atmospheres()
    }

    // not enough partial pressure with oxygen
    fn hypoxia(&self) -> bool {
        // The minimum safe lower limit for the partial pressures of oxygen in a gas mixture is 0.16 bars
        0.16 > self.o2_part_pressure()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hypoxia_false() {
        let habitat = Habitat {
            volume: 1,
            o2: 1.1314 * 1.0 * 0.2,
            n2: 1.16 * 1.0 * 0.8,
            co2: 0.0,
            k_pa: 101.325,
            temperature: 20.0,
            person_count: 0,
        };
        let o2_part_pressure = habitat.o2_part_pressure();
        assert_eq!(o2_part_pressure, 0.22628);
        let hypoxia = habitat.hypoxia();
        assert_eq!(false, hypoxia);
    }

    #[test]
    fn test_hypoxia_true() {
        let habitat = Habitat {
            volume: 1,
            o2: 1.1314 * 1.0 * 0.10,
            n2: 1.16 * 1.0 * 0.8,
            co2: 0.0,
            k_pa: 101.325,
            temperature: 20.0,
            person_count: 0,
        };
        let hypoxia = habitat.hypoxia();
        assert_eq!(true, hypoxia);
    }
}
