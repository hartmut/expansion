// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2017  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// implementation of Storage

// use serde_json;
use self::StorageType::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum StorageType {
    Energy, // in kwh
    Liquid, // in l
    Gas,    // in l
    Solid,  // in kg
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Store {
    storagetype: StorageType, // what type of is stored in this Store
    amount: u64,              // in kwh or l
    mass: u64,                // in kg
}

impl Default for Store {
    fn default() -> Store {
        Store {
            storagetype: Energy,
            amount: 0,
            mass: 0,
        }
    }
}
