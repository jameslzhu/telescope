extern crate linenoise;

mod list;

use std::vec;
use list::List;

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

    let asdf = List::<i32>::new();
    println!("{:#?}", asdf);

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
