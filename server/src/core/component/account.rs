// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

// Money
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Account {
    credits: u64,
}

impl Account {
    pub fn new(credits: u64) -> Self {
        Account { credits }
    }
}
