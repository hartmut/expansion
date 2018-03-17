// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
//

#![allow(dead_code)]
#![allow(unused_variables)]
#[cfg(test)]
mod tests {
    use serde_json;
    use serde_json::Error;
    // use serde::Serialize;
    use common::myuuid::*;
    // use common::stdtrait::StdTrait;
    use common::fileoperations::*;
    use recipes::recipe::*;
    // use recipes::elements::*;
    use recipes::components::*;
    use std::collections::HashMap;
    use rand;
    use rand::distributions::{IndependentSample, Range};

    fn create_one_recipe() -> Recipe {
        let mut rng = rand::thread_rng();
        let between = Range::new(1000f64, 10000.);

        let output_component = Component {
            uuid: get_new_uuid(),
            name: "cheap Solar Panel".to_string(),
            weight: between.ind_sample(&mut rng),
            volume: 5.0,
            prodfrom_recipe_uuid: uuidnull(),
        };

        let input_component = Component {
            uuid: get_new_uuid(),
            name: "Silicon".to_string(),
            weight: 1.0,
            volume: 0.1,
            prodfrom_recipe_uuid: uuidnull(),
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
        new_recipe
    }

    #[test]
    fn create_recipe_example_and_write_to_file() {
        let new_recipe = create_one_recipe();

        // and now write it
        let mut g = newlinewriter("src/tests/testdataout/recipetestout.json".to_string());
        // TODO rewrite to use serialize of recipe itself
        let lineout = serde_json::to_string(&new_recipe).unwrap();
        writerecord(&mut g, &lineout);
        closefile(&mut g);
    }

    //TODO find the bug, non deterministic fail of this test
    // #[test]
    // fn create_recipe_hashmap_and_write_to_file() {
    //     let mut outhash: RecipeHashMap = HashMap::new();
    //     let new_recipe = create_one_recipe();
    //     outhash.insert(new_recipe.get_uuid(), new_recipe);
    //     let new_recipe = create_one_recipe();
    //     outhash.insert(new_recipe.get_uuid(), new_recipe);
    //
    //     // and now write it
    //     let mut g = newlinewriter("src/tests/testdataout/recipetestouthash.json".to_string());
    //     let lineout = serde_json::to_string(&outhash).unwrap();
    //     writerecord(&mut g, &lineout);
    //     closefile(&mut g);
    // }

    #[test]
    pub fn read_recipe_hashmap_file() {
        // read the json file and convert it to a hashmap of recipes
        let result =
            read_file_to_string("src/tests/testdataout/recipetestouthash.json".to_string());
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
    }

}
