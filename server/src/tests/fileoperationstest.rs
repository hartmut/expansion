// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

#[cfg(test)]
mod tests {
    use common::fileoperations::*;

    #[test]
    fn readfile() {
        let mut f = newreader("src/tests/testdata/testfile".to_string());
        // let mut result = String::new();

        let result = getline(&mut f).unwrap();
        assert_eq!(result, "This is something");

        let result = getline(&mut f).unwrap();
        assert_eq!(result, "This is something else");

    }

    // #[test]
    // fn writefile_with_newline() {
    //     let mut f = newlinewriter("src/tests/testdata/testfileout".to_string());
    //     let lineout = "This is something\n".to_string();
    //     writeline(&mut f, &lineout);
    //     closefile(&mut f);
    //
    //     let mut g = newreader("src/tests/testdata/testfileout".to_string());
    //     let result = getline(&mut g).unwrap();
    //     assert_eq!(result, lineout);
    // }

    #[test]
    fn writefile_without_newline() {
        let mut f = newlinewriter("src/tests/testdata/testfileout".to_string());
        let lineout = "This is something".to_string();
        writeline(&mut f, &lineout);
        closefile(&mut f);

        let mut g = newreader("src/tests/testdata/testfileout".to_string());
        let result = getline(&mut g).unwrap();
        assert_eq!(result, lineout);
    }
}
