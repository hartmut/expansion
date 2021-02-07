// macros and plugins
#![allow(dead_code)]
#![warn(unused_variables)]
#![warn(unused_mut)]
extern crate chrono;
extern crate measurements;
extern crate rand;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate specs;
extern crate time;
extern crate toml;
extern crate uuid;

// describe mods to use
pub mod character;
pub mod utils;
pub mod core;
pub mod recipes;
pub mod structure;
pub mod tests;
