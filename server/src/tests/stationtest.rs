// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
//

// standard test player is Ian Banks with uuid 96ff7368-c559-443b-a0c2-0c1324e63cbe
// standard test station is Firefly with uuid 7da4a015-eea9-4a62-aeac-e458910b7b6a

#![allow(dead_code)]
#![allow(unused_variables)]
#[cfg(test)]
mod tests {
    use common::fileoperations::*;
    use common::myuuid::*;
    use common::stdtrait::StdTrait;
    use serde_json;
    use structure::station::AStation;

    #[test]
    fn create_teststation() {
        // create station
        let owner = ExpUuid::parse_str("96ff7368-c559-443b-a0c2-0c1324e63cbe").unwrap();
        let my_station = AStation::new("Firefly".to_string(), owner);

        // write station
        let mut g = newlinewriter("src/tests/testdataout/stationtestout.json".to_string());
        let lineout = <AStation as StdTrait<AStation>>::serialize(&my_station);
        let i = writerecord(&mut g, &lineout);
    }

    #[test]
    fn serialize_test() {
        let owner = ExpUuid::parse_str("96ff7368-c559-443b-a0c2-0c1324e63cbe").unwrap();
        let my_station = AStation::new("Firefly".to_string(), owner);
        let serialized = my_station.serialize();
        let alternativetempstation: AStation = serde_json::from_str(&serialized).unwrap();
    }

    // TODO test doesn't look  usefull
    #[test]
    fn station_getuuid() {
        let owner = ExpUuid::parse_str("96ff7368-c559-443b-a0c2-0c1324e63cbe").unwrap();
        let my_station = AStation::new("Firefly".to_string(), owner);
        let uuid1 = my_station.getuuid();
        let uuid2 = my_station.getuuid();
        assert_eq!(uuid1, uuid2);
    }

    #[test]
    fn read_write_file() {
        // init
        let mut f = newreader("src/tests/testdatain/stationtestin.json".to_string());
        let mut line = String::new();

        let result = readline(&mut f);

        match result {
            // all bad
            None => print!(""),
            // got something
            Some(x) => line = x,
        }

        println!("{}", line);

        // create an entry
        let tempstation: AStation = <AStation as StdTrait<AStation>>::new_from_deserialized(&line);

        // and now write it
        let mut g = newlinewriter("src/tests/testdataout/stationtestout.json".to_string());
        let lineout = <AStation as StdTrait<AStation>>::serialize(&tempstation);
        let i = writerecord(&mut g, &lineout);

        assert_eq!(line, lineout);
    }
}
