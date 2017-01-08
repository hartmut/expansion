// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
// managing the stations updates

use common::workertrait::*;
use common::fileoperations::*;
use common::myuuid::*;
use common::stdtrait::StdTrait;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use serde_json;
use super::station::AStation;

/// holds the informations for the worker for structures
#[derive(Debug)]
pub struct StructureWorker {
    // structure with 'general worker structure'
    worker_struct: WorkerStruct,
    // persistancefile for stations
    stationfile: String,
    // Btree with stations in it
    stations: BTreeMap<ExpUuid, AStation>,
}

// TODO test deserializaion, transfer this to stdtrait for json records #next
// TODO make a smal test case for mutability of BufReader in two stages
// pub fn read_record(f: &mut BufReader<File>) -> AStation {
//     let mut line = String::new();
//     let testplayer = get_new_uuid();
//     let mut outstation = AStation::new("someone".to_string(), testplayer);
//
//     // read lines until you have a record
//     loop {
//         let result = readline(&mut f);
//
//         match result {
//             // all bad
//             None => break,
//             // got something
//             Some(x) => line = x,
//         }
//
//         outstation = AStation::deserialize_test(&line).unwrap();
//         // let tempstation: Result<AStation, String> =
//         // try!(<AStation as StdTrait<AStation>>::new_from_deserialized(&line));
//
//         // match tempstation {
//         //     OK(Station) => outstation = Station,
//         //     Err(err) => println!("didn't work"),
//         // }
//
//     }
//     outstation
// }

impl WorkerTrait<StructureWorker> for StructureWorker {
    fn new(name: String, filename: String) -> StructureWorker {

        let btree: BTreeMap<ExpUuid, AStation> = BTreeMap::new();

        StructureWorker {
            worker_struct: WorkerStruct { name: name },
            stationfile: filename,
            stations: btree,
        }
    }

    fn initialize(&mut self) {

        // init
        let mut f = newreader(self.stationfile.clone());
        let mut line = String::new();

        // iterate over all lines
        loop {

            let result = readline(&mut f);

            match result {
                // all bad
                None => break,
                // got something
                Some(x) => line = x,
            }

            // create an entry
            let tempstation: AStation = <AStation as StdTrait<AStation>>::new_from_deserialized(&line);
            let uuid = tempstation.getuuid();
            self.stations.insert(uuid, tempstation);

            // cleanup
            line.clear();
        }

    }

    fn run(&mut self) {}

    fn send_update(&self) {}

    fn get_update(&mut self) {}
}
