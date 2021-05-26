// macros and plugins
#![allow(dead_code)]
#![warn(unused_variables)]
#![warn(unused_mut)]
extern crate amethyst;
extern crate chrono;
extern crate csv;
extern crate log;
extern crate measurements;
extern crate rand;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate time;
extern crate toml;
extern crate uuid;

// describe mods to use
pub mod core;
pub mod init;
pub mod recipes;
pub mod structure;
pub mod tests;
pub mod utils;
