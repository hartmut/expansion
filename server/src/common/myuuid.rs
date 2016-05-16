// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
//! UUID generator, use uuid crate from https://crates.io/crates/uuid
//! mask the complexity of the crate with an internal api

use uuid::Uuid;

pub fn get_new_uuid() -> Uuid {
    Uuid::new_v4()
}
