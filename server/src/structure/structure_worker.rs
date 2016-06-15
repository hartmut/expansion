// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
//! managing the stations updates

use common::workertrait::*;
use common::readfile::*;
use common::myuuid::*;
use std::collections::BTreeMap;
use super::station::AStation;

/// this structure holds the informations for the worker in the player area
#[derive(Debug)]
pub struct StructureWorker {
    /// structure with 'general worker structure'
    worker_struct: WorkerStruct,
    //vec with stations in it
    stations: BTreeMap<ExpUuid, AStation>,
}

// impl StructureWorker {
//
// }

impl WorkerTrait<StructureWorker> for StructureWorker {
    fn new (name: String) -> StructureWorker {

        let mut btree: BTreeMap<ExpUuid, AStation> = BTreeMap::new();

        StructureWorker {
            worker_struct: WorkerStruct{name: name},
            stations: btree,
        }
    }

    fn initalize (&mut self, filename: String) {
        let f = newreader(filename);
        printfile(f);

    }

    fn send_update (&self) {

    }

    fn get_update (&mut self) {

    }
}
