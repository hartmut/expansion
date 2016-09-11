// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// definitino of receipes, generalized for all types of receipes like research and production
// first step: write it for normal production

// uses
use std::sync::Arc;
use uuid::Uuid;
use package;

//! the package
//! material and quantity
struct Package {
    // pointer at material
    quantity: u64,   // how much
    material: ExpUuid,  //what
}

//! my receipes
struct Recipe {
    uuid: ExpUuid,          // uuid for this receipe
    name: String,           // name of this receipe, something like Air with efficency x%
    duration: u32,          // ticks until the recipe got one run
    input: Arc<Package>,    // vector of UUIDs of materials with how much I need to produce
    output: Arc<Package>,   // vector of UUIDs of materials with how much I will produce
}
