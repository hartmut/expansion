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
        assert_eq!(result, "This is something\n");

        let result = getline(&mut f).unwrap();
        assert_eq!(result, "This is something else\n");

    }

    #[test]
    fn writefile() {
        let mut f = newlinewriter("src/tests/testdata/testfileout".to_string());
        let mut lineout =  "This is something\n".to_string();
        writeline(&mut f, &lineout);
        closefile(&mut f);

        let mut g = newreader("src/tests/testdata/testfileout".to_string());
        let result = getline(&mut g).unwrap();
        assert_eq!(result, lineout);
    }
}
