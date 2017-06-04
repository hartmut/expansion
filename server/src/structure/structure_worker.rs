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

impl WorkerTrait<StructureWorker> for StructureWorker {
    fn new(name: String, stationfile: String) -> StructureWorker {

        let btree: BTreeMap<ExpUuid, AStation> = BTreeMap::new();
        let elementlist: ElementListVec = read_elementlist_file();

        let mut sw = StructureWorker {
            worker_struct: WorkerStruct { name: name },
            stationfile: stationfile,
            stations: btree,
            elementlist: elementlist,
        };

        // general initialization

        // Read stations
        let mut f = newreader(sw.stationfile.clone());
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
            sw.stations.insert(uuid, tempstation);
        }

        sw
    }

    fn step(&mut self) {
        unimplemented!()
    }

    fn send_update(&self) {}

    fn get_update(&mut self) {}
}

#[test]
fn elementlist_dark_matter_check() {
    let sw = StructureWorker::new("Structure_Worker".to_string(),
                                  "src/data/station.json".to_string());
    let e: &Element = &sw.elementlist[0];
    assert!(e.name == "Dark Matter".to_string());
}

#[test]
fn check_station_read_firefly() {
    let sw = StructureWorker::new("Structure_Worker".to_string(),
                                  "src/data/station.json".to_string());
    let station_id = ExpUuid::parse_str("f862e3aa-4a77-4706-88fa-4d03e4e718f2").unwrap();
    let firefly: &AStation = sw.stations.get(&station_id).unwrap();
    assert!(firefly.get_name() == &"Firefly".to_string())
}

#[test]
fn check_station_read_deepspace() {
    let sw = StructureWorker::new("Structure_Worker".to_string(),
                                  "src/data/station.json".to_string());
    let station_id = ExpUuid::parse_str("f962e3aa-4a77-4706-88fa-4d03e4e718f2").unwrap();
    let deepspace: &AStation = sw.stations.get(&station_id).unwrap();
    assert!(deepspace.get_name() == &"Deep Space 9".to_string())
}
