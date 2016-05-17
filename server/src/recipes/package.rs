
// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
//! materials combined with quantity

// uses
use uuid::Uuid;

//! the package
//! material and quantity

struct Package {
    material: Uuid,  //what
    quantity: u64,   // how much
}
