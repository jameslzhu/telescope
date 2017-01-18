#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

extern crate lalrpop_util;

pub mod atom;
pub mod error;
pub mod parse;
