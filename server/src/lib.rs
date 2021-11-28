// macros and plugins
#![allow(dead_code)]
#![warn(unused_variables)]
#![warn(unused_mut)]
extern crate chrono;
extern crate csv;
extern crate measurements;
extern crate serde;
extern crate serde_json;
extern crate toml;
extern crate bevy;
extern crate ron;

// describe mods to use
pub mod core;
pub mod init;
pub mod recipes;
pub mod tests;
pub mod utils;
