// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

#[cfg(test)]
mod tests {
    use crate::core::common::fileoperations::*;

    #[test]
    fn readfile() {
        let mut f = newreader("src/tests/testdatain/testfile".to_string());
        // let mut result = String::new();

        let result = readline(&mut f).unwrap();
        assert_eq!(result, "This is something");

        let result = readline(&mut f).unwrap();
        assert_eq!(result, "This is something else");
    }

    #[test]
    fn writefile_without_newline() {
        let mut f = newlinewriter("src/tests/testdataout/testfileout".to_string());
        let lineout = "This is something".to_string();
        writerecord(&mut f, &lineout);
        closefile(&mut f);

        let mut g = newreader("src/tests/testdataout/testfileout".to_string());
        let result = readline(&mut g).unwrap();
        assert_eq!(result, lineout);
    }
}
