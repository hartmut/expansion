// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2017  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// implementation of Storage

use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Store {
    energy: u64, // in kwh
}

impl Store {
    pub fn new_empty() -> Store {
        Store { energy: 0 }
    }
}
