extern crate nom;

use std::str;

use nom::IResult;
use nom::{digit, multispace};

use list::{Sym, Node, List};

named!(sign <i64>, alt!(
    tag!("-") => { |_| -1 } |
    tag!("+") => { |_|  1 }
));

named!(unsigned<i64>,
    map_res!(
        map_res!(
            delimited!(opt!(multispace), digit, opt!(multispace)),
            str::from_utf8
        ),
        str::FromStr::from_str
    )
);


named!(pub numeric<Node<i64> >,
    chain!(
        s : sign? ~
        d : unsigned,
        || { Node::Num(s.unwrap_or(1) * d) }
    )
);

#[test]
fn test_numeric() {
    let input = "  01234 ";

    if let IResult::Done(_, result) = numeric(input.as_bytes()) {
        assert_eq!(result, Node::Num(1234));
    }

    let input = " -01234 ";
    if let IResult::Done(_, result) = numeric(input.as_bytes()) {
        assert_eq!(result, Node::Num(-1234));
    }
}

named!(pub symbol<Node<i64> >,
    map!(
        delimited!(
            opt!(multispace),
            alt!(
                tag!("+") => {|_| Sym::Add } |
                tag!("-") => {|_| Sym::Sub } |
                tag!("*") => {|_| Sym::Mul } |
                tag!("/") => {|_| Sym::Div }
            ),
            opt!(multispace)
        ),
        |s| { Node::Sym(s) }
    )
);

#[test]
fn test_symbol() {
    let input = " + ";

    if let IResult::Done(_, result) = symbol(input.as_bytes()) {
        assert_eq!(result, Node::Sym(Sym::Add));
    }
}

named!(pub list<List<i64> >,
    delimited!(
        tag!("("),
        map!(
            many0!(
                alt!(
                    symbol | numeric
                )
            ),
            |v| { From::from(v) }
        ),
        tag!(")")
    )
);

#[test]
fn test_list() {
    let input = "( + 1 2 3)";

    let mut list = List::<i64>::new();
    list.elems.push(Node::Sym(Sym::Add));
    list.elems.push(Node::Num(1i64));
    list.elems.push(Node::Num(2i64));
    list.elems.push(Node::Num(3i64));

    let node = Node::List(list);

    if let IResult::Done(_, result) = symbol(input.as_bytes()) {
        assert_eq!(result, node);
    }
}