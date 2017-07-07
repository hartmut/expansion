// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// UUID generator, use uuid crate from https://crates.io/crates/uuid
// mask the complexity of the crate with an internal api

use uuid::Uuid;

pub type ExpUuid = Uuid;
pub const EN_UUID: &'static str = "00000000-0000-0000-0000-000000000001";

pub fn energy_uuid() -> ExpUuid {
    Uuid::parse_str(EN_UUID).unwrap()
}

pub fn get_new_uuid() -> ExpUuid {
    ExpUuid::new_v4()
}

pub fn uuidnull() -> ExpUuid {
    ExpUuid::nil()
}

// TODO define numbergroups -> array with types, enum of types, function for uuids by type
