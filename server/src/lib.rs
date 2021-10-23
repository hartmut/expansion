// macros and plugins
#![allow(dead_code)]
#![warn(unused_variables)]
#![warn(unused_mut)]
extern crate chrono;
extern crate csv;
extern crate measurements;
extern crate rand;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate time;
extern crate toml;
extern crate uuid;
// COMEBACK migrate to bevy
extern crate bevy;

// describe mods to use
pub mod core;
pub mod init;
pub mod recipes;
pub mod structure;
pub mod tests;
pub mod utils;
