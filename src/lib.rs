#![recursion_limit = "1024"]
#[cfg(test)]
extern crate float_cmp;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate combine;

mod ast;
mod token;
mod ops;

pub mod lexer;
pub mod parser;
pub mod error;
