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

#[macro_use]
extern crate lazy_static;

extern crate itertools;
extern crate unicode_xid;

extern crate rustyline;

mod token;
mod ops;
mod forms;
mod repl;

mod ast;
mod lexer;
mod parser;
pub mod error;
pub use repl::repl;
