extern crate linenoise;
extern crate combine;
extern crate combine_language;

mod token;
mod parse;

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
                // let parsed = parser(parse::expr)
                //                  .parse(input.as_str())
                //                  .map(|t| t.0);
                match parsed {
                    Ok(result) => println!("{}", result),
                    Err(e) => println!("error: {}", e),
                }
            }
        };
    }
}
