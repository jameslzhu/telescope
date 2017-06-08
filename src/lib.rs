#![recursion_limit = "1024"]
#[cfg(test)]
extern crate float_cmp;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[macro_use]
extern crate error_chain;

extern crate combine;

pub mod ast;
pub mod token;
pub mod lexer;
pub mod parser;
pub mod error;
pub mod ops;
