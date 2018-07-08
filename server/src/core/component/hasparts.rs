// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use specs;
use std::cmp::Ordering;
use core::common::stdenums::*;

// structure describing each part
#[derive(Debug)]
pub struct Part {
    objtype: ObjType,
    index: specs::world::Index,
}

pub type PartsType = Vec<Part>;

// Parts of a bigger structure
// use in stations and modules
#[derive(Debug)]
pub struct HasParts {
    parts: PartsType,
}

impl HasParts {
    pub fn new() -> Self {
        let parts = PartsType::new();
        HasParts { parts }
    }

    pub fn add(&mut self, objtype: ObjType, index: specs::world::Index){
        if self.find(index) == None {
            self.parts.push(Part{objtype, index});
            self.parts.sort();
        }
    }

    pub fn find(&self, id: specs::world::Index) -> Option<usize> {
        let result = self.parts.iter().position(|&ref r| r.index == id);
        result
    }

}

impl specs::Component for HasParts {
    type Storage = specs::VecStorage<HasParts>;
}

//impl traits for Playerstructpair
impl PartialEq for Part {
    fn eq(&self, other: &Part) -> bool {
        if self.index == other.index { true } else { false }
    }
}

impl Eq for Part {}

impl PartialOrd for Part {
    fn partial_cmp(&self, other: &Part) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// first sort for Parts
impl Ord for Part {
    fn cmp(&self, other: &Part) -> Ordering {
        if self.eq(other) {
            return Ordering::Equal
        }
            else
        {
            if self.index > other.index
                { Ordering::Greater }
            else
                { Ordering::Less }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use specs::world::Builder;

    #[test]
    fn create_partof_component() {
        let mut world = specs::World::new();
        world.register::<HasParts>();
        world.create_entity().with(HasParts::new()).build();
    }

    fn create_testdata() -> HasParts {
        let mut part = HasParts::new();
        //record 3
        part.add(ObjType::Owner, 1);
        // record shouldn't be added
        part.add(ObjType::Owner, 1);
        //record Station
        part.add(ObjType::Station, 1000);
        //record Module
        part.add(ObjType::Module, 1002);
        //record Module
        part.add(ObjType::Module, 2003);
        part
    }

    #[test]
    fn get_vecindex_of_part() {
        let testdata = create_testdata();
        let part = testdata.find(1);
        assert_eq!(part, Some(0));
        let part = testdata.find(1000);
        assert_eq!(part, Some(1));
        let part = testdata.find(1002);
        assert_eq!(part, Some(2));
        let part = testdata.find(2003);
        assert_eq!(part, Some(3));
    }

    #[test]
    fn binary_search_parts() {
        let testdata = create_testdata();
        let record = Part {
            objtype: ObjType::Owner,
            index: 1,
        };
        let element = testdata.parts.binary_search(&record);
        assert_eq!(element, Ok(0));
    }

    #[test]
    fn get_vecindex_for_non_existing_index() {
        let testdata = create_testdata();
        let offrecord = testdata.find(25);
        assert_eq!(None, offrecord);
    }

    // #[test]
    // fn add_structure_deny_when_structure_already_exists() {
    //     let psi = create_testdata();
    //     let record = Playerstructpair {
    //         player: 2,
    //         structure: 2003,
    //     };
    //     let element = psi.psi.binary_search(&record);
    //     assert_eq!(element, Ok(2));
    // }

    // #[test]
    // fn playerstructindex_sorttest() {
    //     //init
    //     let mut psi = Playerstructindex::new();
    //     psi.add_station(100, 1001);
    //     psi.add_station(2, 1002);
    //     //sorting ok?
    //     let result = psi.psi[0] < psi.psi[1];
    //     assert_eq!(result, true);
    //     // one more element
    //     psi.add_station(2, 2003);
    //
    //     //is record 0 what we expect?
    //     let result = &psi.psi[0];
    //     let record = Playerstructpair {
    //         player: 2,
    //         structure: 1002,
    //     };
    //     assert_eq!(*result, record);
    //     //is record 1 what we expect?
    //     let result = &psi.psi[1];
    //     let record = Playerstructpair {
    //         player: 2,
    //         structure: 2003,
    //     };
    //     assert_eq!(*result, record);
    //     //is record 2 what we expect?
    //     let result = &psi.psi[2];
    //     let record = Playerstructpair {
    //         player: 100,
    //         structure: 1001,
    //     };
    //     assert_eq!(*result, record);
    // }
    // #[test]
    // fn playerstructpair_eq() {
    //     let left = Playerstructpair {
    //         player: 1,
    //         structure: 1000,
    //     };
    //     let right = Playerstructpair {
    //         player: 1,
    //         structure: 1000,
    //     };
    //     assert_eq!(left, right);
    //     let result = left == right;
    //     assert_eq!(result, true);
    // }
    // #[test]
    // fn playerstructpair_neq_player() {
    //     let left = Playerstructpair {
    //         player: 1,
    //         structure: 1000,
    //     };
    //     let right = Playerstructpair {
    //         player: 2,
    //         structure: 1000,
    //     };
    //     assert_ne!(left, right);
    // }
    // #[test]
    // fn playerstructpair_neq_structure() {
    //     let left = Playerstructpair {
    //         player: 1,
    //         structure: 1000,
    //     };
    //     let right = Playerstructpair {
    //         player: 1,
    //         structure: 2001,
    //     };
    //     assert_ne!(left, right);
    // }
    // #[test]
    // fn playerstructpair_order_player() {
    //     let left = Playerstructpair {
    //         player: 3,
    //         structure: 1000,
    //     };
    //     let right = Playerstructpair {
    //         player: 2,
    //         structure: 2000,
    //     };
    //     let result = left < right;
    //     assert_eq!(result, false);
    //     let result = left > right;
    //     assert_eq!(result, true);
    // }
    // #[test]
    // fn playerstructpair_order_structure() {
    //     let left = Playerstructpair {
    //         player: 1,
    //         structure: 1000,
    //     };
    //     let right = Playerstructpair {
    //         player: 1,
    //         structure: 2000,
    //     };
    //     let result = left < right;
    //     assert_eq!(result, true);
    //     let result = left > right;
    //     assert_eq!(result, false);
    // }
}
