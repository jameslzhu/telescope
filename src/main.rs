#[macro_use]
extern crate nom;
extern crate combine;
extern crate linenoise;

mod list;
mod parsercombine;
mod parser;

use nom::IResult;

use list::{Sym, Node, List};
use list::Sym::*;

fn main() {
    println!("lisp-rs");
    while let Some(input) = linenoise::input("> ") {
        linenoise::history_add(&input);

        match &input as &str {
            "clear" => linenoise::clear_screen(),
            "exit"  => break,
            _ => {
                let parsed = parser::list(input.as_bytes());
                match parsed {
                    IResult::Done(_, result) => println!("{}", result),
                    IResult::Error(_)        => println!("error: something exploded"),
                    IResult::Incomplete(_)   => println!("error: incomplete")
                }
            }
        };
    }
}
