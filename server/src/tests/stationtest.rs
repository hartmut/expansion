// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
//

#[cfg(test)]
mod tests {
    use serde_json;
    use common::myuuid::*;
    use common::stdtrait::StdTrait;
    use common::fileoperations::*;
    use structure::station::AStation;

    #[test]
    // #[test]
    // fn create_testdata() {
    //     let mut g = newlinewriter("src/tests/testdata/stationtestout.json".to_string());
    //     let mut lineout =  <AStation as StdTrait<AStation>>::serialize(&tempstation);
    //     let i = writeline(&mut g, &lineout);
    // }

    fn serialize_test() {
        let owner = ExpUuid::parse_str("54258d72-dacc-4ee9-ac87-0a0276dda7a6").unwrap();
        let mut my_station = AStation::new("Firefly".to_string(), owner);
        let serialized = my_station.serialize();
        let alternativetempstation: AStation = serde_json::from_str(&serialized).unwrap(); // whats easier - this
    }

    #[test]
    fn read_write_file() {
        //init
        let mut f = newreader("src/tests/testdata/stationtestin.json".to_string());
        let mut line = String::new();

        let result = getline(&mut f);

        match result {
            // all bad
            None => print!(""),
            // got something
            Some(x) => line = x,
        }

        // create an entry
        let tempstation: AStation = <AStation as StdTrait<AStation>>::new_from_deserialized(&line);

        // and now write it
        let mut g = newlinewriter("src/tests/testdata/stationtestout.json".to_string());
        let mut lineout =  <AStation as StdTrait<AStation>>::serialize(&tempstation);
        let i = writeline(&mut g, &lineout);

        assert_eq!(line, lineout+"\n");
    }
}
