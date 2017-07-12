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

use clap::{App, Arg};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::from_usage("-i --interactive 'Run in interactive mode'"))
        .arg(Arg::from_usage("[input] 'Input file'"))
        .get_matches();
    
    if let Some(file) = matches.value_of("input") {
        // Do nothing at the moment
    }
    
    // Run REPL if -i flag supplied or no arguments
    if matches.is_present("interactive") || !matches.is_present("input") {
        let _ = repl::repl();
    }
}
