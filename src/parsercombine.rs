extern crate combine;

use combine::*;
use combine::combinator::FnParser;
use combine::primitives::{Stream};

use list::{Node, List};
use list::Sym::{self, Add, Sub, Mul, Div};

pub fn numeric<I>(input: State<I>) -> ParseResult<Node<i32>, I> where I: Stream<Item=char> {
    (
        optional(choice([token('+'), token('-')])),
        many1(digit()).map(|string: String| string.parse::<i32>().unwrap())
    )
        .map(|(sign, digits)| Node::Num(if sign == Some('-') {-digits} else {digits}))
        .parse_state(input)
}

#[test]
fn test_numeric() {
    let result = parser(numeric).parse("1000");
    assert_eq!(result, Ok((Node::Num(1000), "")));
}

pub fn symbol<I>(input: State<I>) -> ParseResult<Node<i32>, I> where I: Stream<Item=char> {
    choice([
        token('+'),
        token('-'),
        token('*'),
        token('/')
    ])
        .map(|sym| {
            Node::Sym(match sym {
                '+' => Add,
                '-' => Sub,
                '*' => Mul,
                '/' => Div,
                _ => unreachable!()
            })
        })
        .parse_state(input)
}

#[test]
fn test_symbol() {
    let result = parser(symbol).parse("+1");
    assert_eq!(result, Ok((Node::Sym(Add), "1")));

}

// pub fn list<I>(input: State<I>) -> ParseResult<Node<i32>, I> where I: Stream<Item=char> {
//     between(token('('), token(')'), )
// }