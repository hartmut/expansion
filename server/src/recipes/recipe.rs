// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// definitino of recipes, generalized for all types of recipes like research and production
// first step: write it for normal production

// uses
use common::myuuid::*;
use common::stdtrait::StdTrait;
use common::fileoperations::*;
use super::elements::*;
use super::components::*;
use serde_json;
use std::collections::HashMap;

// one Bundle, material or element and quantity
#[derive(Serialize, Deserialize, Debug)]
pub struct Bundle {
    pub quantity: u64, // how much
    pub component: Component, // either a component or
    pub element_no: u32, // a Element
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
    pub uuid: ExpUuid, // uuid for this recipe
    pub uuid_origin: ExpUuid, // Origin of this recipe, if this is an original recipe it has the value "0"
    pub recipe_type: RecipeType, // what type will be produced?
    pub name: String, // name of this recipe
    pub duration: u32, // hours until the recipe produces one set of outputs
    pub input: Vec<Bundle>, // vector of UUIDs of materials and quantity needed to produce the result
    pub output: Vec<Bundle>, // vector of UUIDs of materials and quantity produced, empty if it is a module
    pub json_create: String, // json format for creation of a new module, empty if it is not a module
}

pub type RecipeHashMap = HashMap<ExpUuid, Recipe>;

// TODO flesh out read_recipe_file
pub fn read_recipe_file(filename: String) -> RecipeHashMap {
    let recipehash: RecipeHashMap = HashMap::new();

    recipehash
}

impl Recipe {
    fn get_duration(&self) -> u32 {
        self.duration
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_json_create_def(&self) -> String {
        self.json_create.clone()
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

    // TODO function which returns an produced module

    // TODO function improve recipe
}

impl StdTrait<Recipe> for Recipe {
    fn getuuid(&self) -> ExpUuid {
        self.uuid
    }

    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn new_from_deserialized(input: &String) -> Recipe {
        serde_json::from_str(&input).unwrap()
    }

    fn step(&mut self) {
        // count up ticks should be done in the proucing module
        // is there anything we could do here?
        unimplemented!()
    }
}

#[test]
fn create_recipe_example() {

    let output_component = Component {
        uuid: get_new_uuid(),
        name: "cheap Solar Panel".to_string(),
        weight: 1000.0,
        volume: 5.0,
        receipe_uuid: uuidnull(),
    };

    let input_component = Component {
        uuid: get_new_uuid(),
        name: "Silicon".to_string(),
        weight: 1.0,
        volume: 0.1,
        receipe_uuid: uuidnull(),
    };


    let input_bundle = Bundle {
        quantity: 1000,
        component: input_component,
        element_no: 0,
    };

    let output_bundle = Bundle {
        quantity: 1,
        component: output_component,
        element_no: 0,
    };

    // create a standard module
    let mut new_recipe = Recipe {
        uuid: get_new_uuid(),
        uuid_origin: uuidnull(),
        recipe_type: RecipeType::Energy,
        name: "cheap Solar Panel".to_string(),
        duration: 48,
        input: Vec::<Bundle>::new(),
        output: Vec::<Bundle>::new(),
        json_create: "".to_string(),
    };

    // and put something into input and output
    new_recipe.input.push(input_bundle);
    new_recipe.output.push(output_bundle);

    // and now write it
    let mut g = newlinewriter("src/tests/testdataout/recipetestout.json".to_string());
    let lineout = Recipe::serialize(&new_recipe);
    writerecord(&mut g, &lineout);
}
