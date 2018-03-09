// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use specs;

// Part of a bigger structure
#[derive(Debug)]
pub struct O2 {
    o2prod: u64,
    o2use: u64,
}

impl O2 {
    pub fn new() -> Self {
        O2 { o2prod : 0, o2use: 0 }
    }

    pub fn change_o2use(&mut self, o2new: u64) {
        self.o2use = o2new;
    }

    pub fn change_o2prod(&mut self, o2new: u64) {
        self.o2prod = o2new;
    }
}

impl specs::Component for O2 {
    type Storage = specs::VecStorage<Self>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand;
    use rand::distributions::{IndependentSample, Range};

    #[test]
    fn create_o2_component() {
        let mut world = specs::World::new();
        world.register::<O2>();
        world.create_entity()
            .with(O2::new());
    }

    #[test]
    fn change_o2use(){
        let mut rng = rand::thread_rng();
        let between = Range::new(10u64, 100);
        let sample = between.ind_sample(&mut rng);
        let mut o2 = O2::new();
        o2.change_o2use(sample);
        assert_eq!(o2.o2use, sample);
    }

    #[test]
    fn change_o2prod(){
        let mut rng = rand::thread_rng();
        let between = Range::new(10u64, 100);
        let sample = between.ind_sample(&mut rng);
        let mut o2 = O2::new();
        o2.change_o2prod(sample);
        assert_eq!(o2.o2prod, sample);
    }

}
