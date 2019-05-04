// macros and plugins
#![allow(dead_code)]
#![warn(unused_variables)]
// #![warn(unused_mut)]
// #![feature(proc_macro)]
#[macro_use]

// extern
extern crate chrono;
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
pub mod common;
pub mod core;
pub mod physic;
pub mod recipes;
pub mod structure;
pub mod tests;
