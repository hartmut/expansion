// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
/// helpfull macros found on the web

// https://users.rust-lang.org/t/my-gamedever-wishlist-for-rust/2859/2
macro_rules! max {
     ($e: expr) => { $e };
     ($e: expr, $($rest: tt)*) => { max($e, max!($($rest)*)) }
}

// derivation of macro above
macro_rules! min {
     ($e: expr) => { $e };
     ($e: expr, $($rest: tt)*) => { min($e, min!($($rest)*)) }
}
