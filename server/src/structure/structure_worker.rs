// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// managing the stations updates

use common::workertrait::*;
use common::configuration::*;
use common::fileoperations::*;
use common::myuuid::*;
use common::stdtrait::StdTrait;
use std::collections::BTreeMap;
// use std::collections::HashMap;
use super::station::*;
use recipes::elements::*;
use recipes::recipe::*;

/// holds the informations for the worker for structures
/// is created out of recipes, when you want to see what modules are available,
/// you need to select the recipes for RecipeType => Module
// TODO recipe list with loading at start etc.
#[derive(Debug)]
pub struct StructureWorker {
    // structure with 'general worker structure'
    worker_struct: WorkerStruct,
    // persistancefile for stations
    stationdata: FileData,
    // Btree with stations in it
    stations: BtreeStations,
    // List of Elements for production
    elementlist: ElementListVec,
    // List of recipes
    recipelist: RecipeHashMap,
}

impl WorkerTrait<StructureWorker> for StructureWorker {
    fn new(name: String, myconfig: &Configuration) -> StructureWorker {

        let btree: BTreeMap<ExpUuid, AStation> = BTreeMap::new();

        let mut sw = StructureWorker {
            worker_struct: WorkerStruct { name: name },
            stationdata: myconfig.get_structure_config(),
            stations: btree,
            elementlist: read_elementlist_file(myconfig.get_elements_config().get_datafile()),
            // TODO read file from config
            recipelist: read_recipe_file("src/data/recipe.json".to_string()),
        };

        // import stations
        let mut f = newreader(sw.stationdata.get_datafile());
        let mut line = String::new();

        // iterate over all stations
        loop {
            match readline(&mut f) {
                // all bad or end of file
                None => break,
                // got something
                Some(x) => line = x,
            }

            // insert the station into the structure of the worker
            let tempstation: AStation = <AStation as StdTrait<AStation>>::new_from_deserialized(&line);
            let uuid = tempstation.getuuid();
            sw.stations.insert(uuid, tempstation);
        }

        sw
    }

    fn step(&mut self) {
        println!("one step in structure worker", );
    }

    fn send_update(&self) {}

    fn get_update(&mut self) {}
}
