// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// definitino of recipes, generalized for all types of recipes like research and production
// first step: write it for normal production

// TODO integrated into new structure

// NOTE change to legion

// uses
use super::components::*;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Error;
use std::collections::HashMap;
use utils::fileoperations::*;
use utils::myuuid::*;

// one Bundle, material or element and quantity
#[derive(Serialize, Deserialize, Debug)]
pub struct Bundle {
    pub quantity: u64,        // how much
    pub component: Component, // either a component or
    pub element_no: u32,      // a Element
}

// types of recipes
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum RecipeType {
    Module,
    Component,
    Research,
    Energy,
}

// my recipes
#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    pub uuid: ExpUuid,           // uuid for this recipe
    pub uuid_origin: ExpUuid, // Origin of this recipe, if this is an original recipe it has the value "0"
    pub recipe_type: RecipeType, // what type will be produced?
    pub name: String,         // name of this recipe
    pub duration: u32,        // hours until the recipe produces one set of outputs
    pub input: Vec<Bundle>, // vector of UUIDs of materials and quantity needed to produce the result
    pub output: Vec<Bundle>, // vector of UUIDs of materials and quantity produced, empty if it is a module
    pub prefab: String, // json format for creation of a new module, empty if it is not a module
}

pub type RecipeHashMap = HashMap<ExpUuid, Recipe>;

pub fn read_recipe_file(filename: String) -> RecipeHashMap {
    // read the json file and convert it to a hashmap of recipes
    let result = read_file_to_string(filename);
    let recipes: Result<RecipeHashMap, Error> = serde_json::from_str(&result);

    // check if the conversion of the elementlist from the json file worked as predicted
    let recipehash: RecipeHashMap = match recipes {
        Ok(recipes) => recipes,
        Err(error) => {
            panic!(
                "somethings is wrong with the deserialization of the recipehashfile: {:?}",
                error
            );
        }
    };

    recipehash
}

impl Recipe {
    pub fn get_uuid(&self) -> ExpUuid {
        self.uuid.clone()
    }

    fn get_duration(&self) -> u32 {
        self.duration
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_json_create_def(&self) -> String {
        self.prefab.clone()
    }

    fn get_recipe_type(&self) -> RecipeType {
        self.recipe_type
    }

    fn is_module(&self) -> bool {
        match self.recipe_type {
            RecipeType::Module => true,
            _ => false,
        }
    }

    fn is_component(&self) -> bool {
        match self.recipe_type {
            RecipeType::Component => true,
            _ => false,
        }
    }

    fn is_research(&self) -> bool {
        match self.recipe_type {
            RecipeType::Research => true,
            _ => false,
        }
    }
}
