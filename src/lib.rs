#![allow(dead_code)]
#[macro_use]
extern crate vec_derive;
#[macro_use]
extern crate vec_macros;
extern crate num;
#[macro_use]
extern crate quote;
extern crate generic_array;

pub use ::vector::*;
pub use ::matrix::*;
mod vector;
mod matrix;
