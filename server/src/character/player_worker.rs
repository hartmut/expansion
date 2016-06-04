// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
/// worker for player


use common::traits;

/// this structure holds the informations for the worker in the player area
pub struct PlayerWorker {
    /// structure with 'general worker structure'
    worker_struct: traits::WorkerStruct,
    //vec with players in it
}

impl PlayerWorker {
    pub fn new (name: String) -> PlayerWorker {
        PlayerWorker {
            worker_struct: traits::WorkerStruct{name: name},
        }
    }
}
