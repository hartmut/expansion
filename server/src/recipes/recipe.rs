// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// definitino of recipes, generalized for all types of recipes like research and production
// first step: write it for normal production

// uses
use std::sync::Arc;
use uuid::Uuid;
use package;

//! the package
//! material and quantity
#[derive(Serialize, Deserialize, Debug)]
struct Package {
    // pointer at material
    quantity: u64,   // how much
    material: ExpUuid,  //what
}

//!types of recipes
#[derive(Serialize, Deserialize, Debug)]
enum RecipeType {
    Module,
    Component,
    Research,
}

//! my recipes
#[derive(Serialize, Deserialize, Debug)]
struct Recipe {
    uuid: ExpUuid,          // uuid for this recipe
    uuid_origin: ExpUuid,   // Origin of this recipe, if this is a starter it has the value "0"
    type: RecipeType,       // what type will be produced?
    name: String,           // name of this recipe, something like Air with efficency x%
    duration: u32,          // ticks until the recipe got one run
    input: Arc<Package>,    // vector of UUIDs of materials with how much I need to produce
    component: Arc<Package>,   // vector of UUIDs of materials with how much I will produce
    module: String,         // json format for creation of a new module
}
