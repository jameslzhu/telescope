extern crate combine;

use combine::*;
use combine::primitives::Stream;

use atom::{Atom, Expr};
use atom::Sym::{Add, Sub, Mul, Div};

pub fn numeric<I>(input: State<I>) -> ParseResult<Atom<i32>, I>
    where I: Stream<Item = char>
{
    (// Sign parser
     optional(choice([token('+'), token('-')])),

     // Digits parser
     many1(digit()).map(|string: String| string.parse::<i32>().unwrap()))
        .map(|(sign, digits)| {
            Atom::Num(if sign == Some('-') { -digits } else { digits })
        })
        .parse_state(input)
}

#[test]
fn test_numeric() {
    let result = parser(numeric).parse("1000");
    assert_eq!(result, Ok((Atom::Num(1000), "")));
}

pub fn symbol<I>(input: State<I>) -> ParseResult<Atom<i32>, I>
    where I: Stream<Item = char>
{
    choice([token('+'), token('-'), token('*'), token('/')])
        .map(|sym| {
            Atom::Sym(match sym {
                '+' => Add,
                '-' => Sub,
                '*' => Mul,
                '/' => Div,
                _ => unreachable!(),
            })
        })
        .parse_state(input)
}

#[test]
fn test_symbol() {
    let result = parser(symbol).parse("+1");
    assert_eq!(result, Ok((Atom::Sym(Add), "1")));
}

pub fn expr<I>(input: State<I>) -> ParseResult<Expr<i32>, I>
    where I: Stream<Item = char>
{
    between(token('(').skip(spaces()),
            token(')'),
            many(parser(symbol).or(parser(numeric)).skip(spaces())))
        .parse_state(input)
}

#[test]
fn test_expr() {
    let result = parser(expr).parse("( 1 2 3 )");

    let expr = Expr::from(vec![1, 2, 3]);

    assert_eq!(result, Ok((expr, "")));
}
