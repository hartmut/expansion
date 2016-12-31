// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
//

// standard tests for modules

#[cfg(test)]
mod tests {
    use serde_json;
    use common::myuuid::*;
    use common::stdtrait::StdTrait;
    use common::fileoperations::*;
    use structure::modules::Module;

    fn writetestdata(input: Module) {
        let mut f = newlinewriter("src/tests/testdataout/moduletestout.json".to_string());
        let lineout = <Module as StdTrait<Module>>::serialize(&input);
        writeline(&mut f, &lineout);
    }

    #[test]
    fn create_module_energyprod() {
        let testmodule = Module::new("testmodule".to_string(),
                                     ExpUuid::parse_str("96ff7368-c559-443b-a0c2-0c1324e63cbe")
                                         .unwrap(),
                                     100,
                                     ExpUuid::nil());
        writetestdata(testmodule);
    }
}
