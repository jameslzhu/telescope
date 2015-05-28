#![feature(convert)]

extern crate linenoise;

fn main() {
    println!("lisp-rs");
    while let Some(input) = linenoise::input("> ") {
        linenoise::history_add(input.as_str());
        match input.as_str() {
            "clear" => linenoise::clear_screen(),
            "exit" => break,
            _ => println!("{}", input),
        };
    }
}
