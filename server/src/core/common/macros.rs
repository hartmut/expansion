// Experimental Simulator of a cooperative solar system economy.
// See doc/LICENSE for licensing information
//
/// helpfull macros

// https://users.rust-lang.org/t/my-gamedever-wishlist-for-rust/2859/2
macro_rules! max {
     ($e: expr) => { $e };
     ($e: expr, $($rest: tt)*) => { max($e, max!($($rest)*)) }
}

// derivation of macro "max" above
macro_rules! min {
     ($e: expr) => { $e };
     ($e: expr, $($rest: tt)*) => { min($e, min!($($rest)*)) }
}
