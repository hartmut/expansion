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
use std::sync::Arc;
use serde_json;
use super::station::AStation;
use recipes::elements::*;

/// holds the informations for the worker for structures
#[derive(Debug)]
pub struct StructureWorker {
    // structure with 'general worker structure'
    worker_struct: WorkerStruct,
    // persistancefile for stations
    stationfile: String,
    // Btree with stations in it
    stations: BTreeMap<ExpUuid, AStation>,
    // List of Elements for production
    elementlist: ElementListVec,
}

// IDEA test deserialization, transfer this to stdtrait for json records
pub fn read_record(mut f: &mut BufReader<File>) -> AStation {
    let mut line = String::new();
    let testplayer = uuidnull();
    let mut outstation = AStation::new("someone".to_string(), testplayer);

    // read lines until you have a record
    // TODO example from tokio *if let Some(n) = buf.as_ref().iter().position(|b| *b == b'\n') {*
    // https://lukesteensen.com/2016/12/getting-started-with-tokio/
    loop {
        let result = readline(&mut f);

        match result {
            // all bad
            None => break,
            // got something
            Some(x) => line = x,
        }

        // test whether the line is a json record of this type

        // outstation = AStation::deserialize_test(&line).unwrap();
        // let tempstation: Result<AStation, String> =
        //     try!(<AStation as StdTrait<AStation>>::new_from_deserialized(&line));
        // match tempstation {
        //     OK(Station) => outstation = Station,
        //     Err(err) => println!("didn't work"),
        // }

    }
    outstation
}

impl WorkerTrait<StructureWorker> for StructureWorker {
    fn new(name: String, filename: String) -> StructureWorker {

        let btree: BTreeMap<ExpUuid, AStation> = BTreeMap::new();
        let elementlist: ElementListVec = Vec::new();

        StructureWorker {
            worker_struct: WorkerStruct { name: name },
            stationfile: filename,
            stations: btree,
            elementlist: elementlist,
        }
    }

    fn initialize(&mut self) {

        // general initialization

        // Read stations
        let mut f = newreader(self.stationfile.clone());
        let mut line = String::new();

        // iterate over all stations
        loop {

            let result = readline(&mut f);

            match result {
                // all bad or end of file
                None => break,
                // got something
                Some(x) => line = x,
            }

            // insert the station
            let tempstation: AStation = <AStation as StdTrait<AStation>>::new_from_deserialized(&line);
            let uuid = tempstation.getuuid();
            self.stations.insert(uuid, tempstation);

            // cleanup
            line.clear();
        }
        // read elementlist
        self.elementlist = read_elementlist_file();
        println!("{:?}", self.elementlist);

    }

    fn run(&mut self) {}

    fn send_update(&self) {}

    fn get_update(&mut self) {}
}
