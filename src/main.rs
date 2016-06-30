#[macro_use]
extern crate error_chain;

extern crate linenoise;
extern crate combine;
extern crate combine_language;

mod token;
mod parse;
mod error;

use combine::*;

fn main() {
    println!("lisp-rs");

    while let Some(input) = linenoise::input("> ") {
        linenoise::history_add(&input);

        match input.as_str() {
            "clear" => linenoise::clear_screen(),
            "exit" => break,
            _ => {
                let parsed = parser(parse::expr)
                    .parse(input.as_str())
                    .map(|t| t.0);

                match parsed {
                    Ok(result) => {
                        println!("{}", result);
                        match result.eval() {
                            Ok(value) => println!("{}", value),
                            Err(e) => println!("{}", e),
                        }
                    },
                    Err(e) => println!("error: {}", e),
                }
            }
        };
    }
}
