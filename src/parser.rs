use ast::{Expr, List, Vector, Symbol};
use combine::{Stream, Parser, ParseError, ParseResult};
use combine::{between, many, many1, parser, satisfy_map, token, try};
use token::Token;

pub fn parse<I>(input: I) -> Result<(Expr, I), ParseError<I>>
where
    I: Stream<Item = Token>,
{
    // many(parser(expr)).parse(input)
    parser(expr).parse(input)
}

fn expr<I>(input: I) -> ParseResult<Expr, I>
where
    I: Stream<Item = Token>,
{
    choice!(parser(atom), parser(symbol), parser(list), parser(vector)).parse_stream(input)
}

fn atom<I>(input: I) -> ParseResult<Expr, I>
where
    I: Stream<Item = Token>,
{
    satisfy_map(|token| if let Token::Literal(lit) = token {
        Some(Expr::from(lit))
    } else {
        None
    }).parse_stream(input)
}

fn list<I>(input: I) -> ParseResult<Expr, I>
where
    I: Stream<Item = Token>,
{
    let nil = token(Token::LParen).and(token(Token::RParen)).map(
        |_| Expr::Nil,
    );
    try(between(
        token(Token::LParen),
        token(Token::RParen),
        many1(parser(expr)).map(List).map(Expr::List),
    )).or(nil)
        .parse_stream(input)
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

fn symbol<I>(input: I) -> ParseResult<Expr, I>
where
    I: Stream<Item = Token>,
{
    satisfy_map(|token| if let Token::Symbol(sym) = token {
        Some(Expr::from(Symbol(sym)))
    } else {
        None
    }).parse_stream(input)
}
