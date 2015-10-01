extern crate combine;

use std::str::FromStr;

use combine::*;
use combine::combinator::FnParser;
use combine::primitives::{Stream};

use list::{Sym, Node, List};

enum Sign { Pos, Neg }

// fn sign<I>(input: State<I>) -> ParseResult<(Sign, String), I> {
//     let mut parser = choice([token('-'), token('+')]
//     match parser.parse(input) {
//         Ok(("-", _)) => Sign::Pos,
//         Ok(("-")) | Err(_) => Sign::Neg,
//     }
// }

fn digits_to_int(digits : Vec<char>) -> i32
{
    digits.iter().fold(0, |acc, &d| acc * 10 + d.to_digit(10).expect("digit") as i32)
}

fn many_digits<I>() -> FnParser<I, fn (State<I>) -> ParseResult<i32, I>>
where I: Stream<Item=char> {

    fn many_digits_<I>(input: State<I>) -> ParseResult<i32, I>
    where I: Stream<Item=char> {
        many1::<Vec<char>, _>(digit())
            .map(digits_to_int)
            .parse_state(input)
    }

    parser(many_digits_)
}

fn numeric<I>(input: State<I>) -> ParseResult<i32, I> where I: Stream<Item=char> {
    (
        optional(choice([token('+'), token('-')])),
        many_digits()
    )
        .map(|(sign, digits)| -> i32 {
            match sign {
                Some('-') => -digits,
                _ => digits
            }
        })
        .parse_state(input)
}

#[test]
fn test() {
    let result = parser(numeric).parse("1000");
    println!("{:?}", result);
    assert_eq!(result, Ok((1000, "")));
}
