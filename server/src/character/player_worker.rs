// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
/// worker for player

use common::workertrait::*;
use common::fileoperations::*;

/// this structure holds the informations for the worker in the player area
#[derive(Debug)]
pub struct PlayerWorker {
    /// structure with 'general worker structure'
    worker_struct: WorkerStruct,
    //vec with players in it
}

impl PlayerWorker {
    pub fn new (name: String, filename: String) -> PlayerWorker {
        let f = newreader(filename);

        PlayerWorker {
            worker_struct: WorkerStruct{name: name},
        }
    }
}
