// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2017  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// implementation of Storage

// use serde_json;
use self::StorageType::*;

#[derive(Serialize, Deserialize, Debug)]
enum StorageType {
    Energy, // in kwh
    Food, // in kg
    Liquid, // in l
    Gas, // in l
    Solid, // in kg
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Store {
    storagetype: StorageType, // what type of is stored in this Store
    amount: u64, // in kwh
    mass: u64, // in kg
}

impl Store {
    pub fn new_empty() -> Store {
        Store {
            storagetype: Energy,
            amount: 0,
            mass: 0,
        }
    }
}
