#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

pub mod atom;
pub mod parse;
pub mod error;
mod ops;
