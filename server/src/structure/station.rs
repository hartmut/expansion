// Experimental Simulator of a cooperative solar system economy.

#[derive(Debug)]
pub struct AStation {
    energyuse: u64,   // energy usage per tick, sum over all modules
    energyprod: u64,  // energy production per tick, sum over all modules
    o2prod: u64,      // production of O2, see above -> people module?
    o2use: u64,       // use of O2 for people
}

