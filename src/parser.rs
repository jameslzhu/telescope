use combine::{Stream, Parser, ParseError, ParseResult};
use combine::{between, many, many1, parser, satisfy_map, token, value};
use token::Token;
use types::{Expr, List, Vector, Symbol};

pub fn parse<I>(input: I) -> Result<(Vec<Expr>, I), ParseError<I>>
where
    I: Stream<Item = Token>,
{
    many(parser(expr)).parse(input)
}

fn expr<I>(input: I) -> ParseResult<Expr, I>
where
    I: Stream<Item = Token>,
{
    parser(atom)
        .or(parser(list))
        .or(parser(vector))
        .parse_stream(input)
}

fn atom<I>(input: I) -> ParseResult<Expr, I>
where
    I: Stream<Item = Token>,
{
    satisfy_map(|token| match token {
        Token::Literal(lit) => Some(Expr::from(lit)),
        Token::Symbol(sym) => Some(Expr::from(Symbol(sym))),
        _ => None,
    }).parse_stream(input)
}

fn list<I>(input: I) -> ParseResult<Expr, I>
where
    I: Stream<Item = Token>,
{
    between(
        token(Token::LParen),
        token(Token::RParen),
        many1(parser(expr)).map(List).map(Expr::List).or(value(
            Expr::Nil,
        )),
    ).parse_stream(input)
}

fn vector<I>(input: I) -> ParseResult<Expr, I>
where
    I: Stream<Item = Token>,
{
    between(
        token(Token::LBracket),
        token(Token::RBracket),
        many(parser(expr)).map(Vector).map(Expr::Vector),
    ).parse_stream(input)
}
