// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// definitino of recipes, generalized for all types of recipes like research and production
// first step: write it for normal production

// uses
use common::myuuid::*;
use common::stdtrait::StdTrait;
// use common::fileoperations::*;
use super::elements::*;
use super::components::*;
use serde_json;

// one Bundle, material or element and quantity
#[derive(Serialize, Deserialize, Debug)]
struct Bundle {
    quantity: u64, // how much
    component: Component, // either a component or
    element: Element, // a Element
}

// types of recipes
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
enum RecipeType {
    Module,
    Component,
    Research,
}

// my recipes
#[derive(Serialize, Deserialize, Debug)]
struct Recipe {
    uuid: ExpUuid, // uuid for this recipe
    uuid_origin: ExpUuid, // Origin of this recipe, if this is an original recipe it has the value "0"
    recipe_type: RecipeType, // what type will be produced?
    name: String, // name of this recipe
    duration: u32, // ticks until the recipe got one run
    input: Vec<Bundle>, // vector of UUIDs of materials and quantity needed to produce the result
    output: Vec<Bundle>, // vector of UUIDs of materials and quantity produced, empty if it is a module
    module: String, // json format for creation of a new module, empty if it is not a module
}

impl Recipe {
    fn get_duration(&self) -> u32 {
        self.duration
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_module_def(&self) -> String {
        self.module.clone()
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
fn create_module_example() {

    let new_bundle1 = Bundle {
        quantity: 1000,
        material: get_new_uuid(),
    };

    let new_bundle2 = Bundle {
        quantity: 0,
        material: get_new_uuid(),
    };


    // create a standard module
    let mut new_module_recipe = Recipe {
        uuid: get_new_uuid(),
        uuid_origin: get_new_uuid(),
        recipe_type: RecipeType::Module,
        name: "Basic Module I".to_string(),
        duration: 100,
        input: Vec::<Bundle>::new(),
        output: Vec::<Bundle>::new(),
        module: "".to_string(),
    };

    // and put something into input and output
    new_module_recipe.input.push(new_bundle1);
    new_module_recipe.output.push(new_bundle2);

    // and now write it
    let mut g = newlinewriter("src/tests/testdataout/recipetestout.json".to_string());
    let lineout = Recipe::serialize(&new_module_recipe);
    writerecord(&mut g, &lineout);
}
