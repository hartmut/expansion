// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// definitino of receipe

// uses
use uuid::Uuid;
use package;

//! my receipes
struct Recipe {
    uuid: ExpUuid,             // uuid for this receipe
    name: String,           // name of this receipe, something like Air with efficency x%
    duration: u32           // ticks until the recipe got one run
    input: Vec<Package>,    // vector of UUIDs of materials with how much I need to produce
    output: Vec<Package>,   // vector of UUIDs of materials with how much I will produce
}
