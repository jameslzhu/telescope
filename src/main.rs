#[macro_use]
extern crate nom;
extern crate linenoise;

mod list;
mod parser;
use list::{Sym, Node, List};
use list::Sym::*;

fn main() {
    let mut asdf = List::<i32>::new();
    asdf.elems.push(Node::Sym(Add));
    asdf.elems.push(Node::Num(1));
    println!("{}", asdf);

    println!("lisp-rs");
    while let Some(input) = linenoise::input("> ") {
        linenoise::history_add(&input);

        match &input as &str {
            "clear" => linenoise::clear_screen(),
            "exit"  => break,
            _ => println!("{}", input),
        };
    }
}
