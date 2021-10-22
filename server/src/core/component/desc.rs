// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

// Descriptions
#[derive(Clone, Debug, PartialEq)]
pub struct Desc {
    pub name: String,
    pub longtext: String,
}

impl Desc {
    pub fn new(name: impl Into<String>, longtext: impl Into <String>) -> Self {
        let name = name.into();
        let longtext = longtext.into();
        Desc { name, longtext }
    }
}
