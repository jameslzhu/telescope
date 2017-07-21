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

mod buffer;
mod types;
mod eval;
mod forms;
mod lexer;
mod parser;
mod ops;
mod token;
mod error;
mod util;
mod input;
mod env;

use clap::{App, Arg};

fn main() {
    use std::sync::Arc;
    println!("Expr: {}", ::std::mem::size_of::<types::Expr>());
    println!("String: {}", ::std::mem::size_of::<String>());
    println!("Symbol: {}", ::std::mem::size_of::<types::Symbol>());
    println!("Arc<Fn>: {}", ::std::mem::size_of::<Arc<types::Function>>());
    println!("Arc<Macro>: {}", ::std::mem::size_of::<Arc<types::Macro>>());
    println!("List: {}", ::std::mem::size_of::<types::List>());
    println!("Vector: {}", ::std::mem::size_of::<types::Vector>());
    println!("Map: {}", ::std::mem::size_of::<types::Map>());

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::from_usage(
            "-i --interactive 'Run in interactive mode'",
        ))
        .arg(Arg::from_usage(
            "[input] 'Read program from file (- for stdin)'",
        ))
        .get_matches();

    let env = ops::env();

    if let Some(file) = matches.value_of("input") {
        if file == "-" {
            
        } else {
            match input::file(file, env.clone()) {
                Ok(_) => (),
                Err(err) => println!("{}", err),
            }
        }
    }

    if !matches.is_present("input") {
        println!("telescope v{}", env!("CARGO_PKG_VERSION"));
    }

    // Run REPL if -i flag supplied or no arguments
    if matches.is_present("interactive") || !matches.is_present("input") {
        match input::repl(env.clone()) {
            Ok(_) => (),
            Err(err) => println!("{}", err),
        }
    }
}
