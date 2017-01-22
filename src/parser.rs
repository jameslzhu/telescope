use combine::*;
use combine::char::*;

use atom::{Symbol, Atom, List, Node, Expr};

fn atom<I>(input: I) -> ParseResult<Atom, I>
    where I: Stream<Item=char>
{
    let num = optional(char('-'))
        .and(many1(digit()))
        .map(|(sign, digits): (Option<char>, Vec<char>)| {
            let magnitude = digits.into_iter()
                .collect::<String>()
                .parse::<i32>().unwrap();
            if sign == Some('-') { -magnitude } else { magnitude }
        })
        .map(Atom::from);

    let sym = satisfy(|c| "+-*/%".chars().any(|x| x == c)) 
        .map(|sym| match sym {
            '+' => Symbol::Add,
            '-' => Symbol::Sub,
            '*' => Symbol::Mul,
            '/' => Symbol::Div,
            '%' => Symbol::Mod,
            _ => unreachable!(),
        })
        .or(choice([string("head"), string("tail")])
            .map(|sym| match sym {
                "head" => Symbol::Head,
                "tail" => Symbol::Tail,
                _ => unreachable!(),
            }))
        .map(Atom::from);

    num.or(sym).parse_stream(input)
}

fn list<I>(input: I) -> ParseResult<List, I>
    where I: Stream<Item=char>
{
    let lex_char = |c| char(c).skip(spaces());
    let space_list = sep_by(parser(node), spaces());

    between(lex_char('['), lex_char(']'), space_list)
        .map(|list: Vec<Node>| List::from(list.as_slice()))
        .parse_stream(input)
}

fn expr<I>(input: I) -> ParseResult<Expr, I>
    where I: Stream<Item=char>
{
    let lex_char = |c| char(c).skip(spaces());
    let space_list = sep_by(parser(node), spaces());

    between(lex_char('('), lex_char(')'), space_list)
        .map(|list: Vec<Node>| Expr::new(list))
        .parse_stream(input)
}

fn node<I>(input: I) -> ParseResult<Node, I>
    where I: Stream<Item=char>
{
    parser(atom).map(Node::from)
        .or(parser(list).map(Node::from))
        .or(parser(expr).map(Node::from))
        .skip(spaces())
        .parse_stream(input)
}

pub fn lang<I>(input: I) -> ParseResult<Node, I>
    where I: Stream<Item=char>
{
    spaces().with(parser(node))
        .parse_stream(input)
}
