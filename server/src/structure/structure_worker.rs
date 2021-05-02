// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// managing the stations updates

use super::station::*;
use recipes::elements::*;
use recipes::recipe::*;
use utils::configuration::*;

/// holds the informations for the worker for structures
/// is created out of recipes, when you want to see what modules are available,
/// you need to select the recipes for RecipeType => Module
#[derive(Debug)]
pub struct StructureWorker {
    // persistancefile for stations
    stationdata: FileData,
    // Btree with stations in it
    stations: BtreeStations,
    // List of Elements for production
    elementlist: ElementListVec,
    // List of recipes
    recipelist: RecipeHashMap,
}
