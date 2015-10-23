#[macro_use]
extern crate nom;
extern crate combine;
extern crate linenoise;

mod list;
mod parsercombine;
mod parser;

use nom::IResult;

use combine::*;

fn main() {
    println!("lisp-rs");
    while let Some(input) = linenoise::input("> ") {
        linenoise::history_add(&input);

        match &input as &str {
            "clear" => linenoise::clear_screen(),
            "exit"  => break,
            _ => {
                let parsed = parser(parsercombine::list)
                    .parse(&input as &str)
                    .map(|t| t.0);
                match parsed {
                    Ok(result) => println!("{}", result),
                    Err(_) => println!("error: something exploded")
                }
                // let parsed = parser::list(input);
                // match parsed {
                //     IResult::Done(_, result) => println!("{}", result),
                //     IResult::Error(_)        => println!("error: something exploded"),
                //     IResult::Incomplete(_)   => println!("error: incomplete")
                // }
            }
        };
    }
}
