#![deny(missing_docs)]
#![deny(warnings)]

//! Data structures and functions shared between server and client.

#![feature(test)]
#![feature(unboxed_closures)]
#![feature(range_inclusive)]
#![feature(iter_cmp)]

extern crate cgmath;
#[macro_use]
extern crate log;
extern crate num;
extern crate test;
extern crate time;
extern crate kafka;

#[macro_use]
extern crate serialize as _serialize;

pub mod object-position;
pub mod communicate;
pub mod entity;
pub mod id_allocator;
pub mod interval_timer;
pub mod lod;
pub mod range_abs;
pub mod socket;

pub use _serialize as serialize;
