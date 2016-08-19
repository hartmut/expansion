// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
//

use structure::station::AStation;
use common::fileoperations::*;
use common::stdtrait::StdTrait;

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
