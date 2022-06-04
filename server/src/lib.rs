// macros and plugins
#![allow(dead_code)]
#![warn(unused_variables)]
#![warn(unused_mut)]
extern crate chrono;
extern crate csv;
extern crate measurements;
extern crate serde;
extern crate serde_json;
extern crate bevy;
extern crate ron;

// describe mods to use
pub mod core;
pub mod init;
pub mod client;
pub mod debug;
pub mod recipes;
pub mod tests;