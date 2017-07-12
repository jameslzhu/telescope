#![recursion_limit = "1024"]
#[cfg(test)]
extern crate float_cmp;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate derive_error_chain;

#[macro_use]
extern crate lazy_static;

extern crate clap;
extern crate combine;
extern crate conv;
extern crate itertools;
extern crate unicode_xid;
extern crate rustyline;

mod ast;
mod eval;
mod forms;
mod lexer;
mod parser;
mod ops;
mod token;
mod repl;
mod error;

fn main() {
    let _ = repl::repl();
}
