extern crate combine;
extern crate linenoise;

mod atom;
mod atom2;
mod list;
mod parse;

use combine::*;

fn main() {
    println!("lisp-rs");
    while let Some(input) = linenoise::input("> ") {
        linenoise::history_add(&input);

        match &input as &str {
            "clear" => linenoise::clear_screen(),
            "exit" => break,
            _ => {
                let parsed = parser(parse::expr)
                                 .parse(&input as &str)
                                 .map(|t| t.0);
                match parsed {
                    Ok(result) => println!("{}", result),
                    Err(_) => println!("error: something exploded"),
                }
            }
        };
    }
}
