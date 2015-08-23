extern crate linenoise;

mod list;
use list::{Sym, Node, List};

// pub struct List {
//     head: Link,
// }
//
// enum Link {
//     Empty,
//     More(Box<Node>),
// }
//
// struct Node {
//     elem: i32,
//     next: Link,
// }

fn main() {
    let mut asdf = List::<i32>::new();
    asdf.elems.push(Node::Sym(Sym::Add));
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
