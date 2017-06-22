use ast::{Atom, Expr, Function, List, Quote, Symbol};
use combine::{Stream, Parser, ParseError, ParseResult};
use combine::{between, many, many1, parser, satisfy_map, token, try};
use ops;
use token::Token;

pub fn parse<I>(input: I) -> Result<(Vec<Expr>, I), ParseError<I>>
    where I: Stream<Item = Token>
{
    many(parser(expr)).parse(input)
}

fn expr<I>(input: I) -> ParseResult<Expr, I>
    where I: Stream<Item = Token>
{
    choice!(
        parser(atom),
        parser(builtin),
        parser(symbol),
        parser(list),
        parser(quote)
    ).parse_stream(input)
}

fn atom<I>(input: I) -> ParseResult<Expr, I>
    where I: Stream<Item = Token>
{
    satisfy_map(|token| {
            if let Token::Literal(lit) = token {
                Some(Expr::from(lit))
            } else {
                None
            }
        })
        .parse_stream(input)
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn builtin<I>(input: I) -> ParseResult<Expr, I>
    where I: Stream<Item = Token>
{
    satisfy_map(|token| {
            match token {
                // Math tokens
                Token::Plus    => Some(Function::new(Some("+"), Box::new(ops::add)).into()),
                Token::Minus   => Some(Function::new(Some("-"), Box::new(ops::sub)).into()),
                Token::Star    => Some(Function::new(Some("*"), Box::new(ops::mul)).into()),
                Token::Slash   => Some(Function::new(Some("/"), Box::new(ops::div)).into()),

                // Comparison tokens
                Token::Equal   => Some(Function::new(Some("="), Box::new(ops::equal)).into()),
                Token::Less    => Some(Function::new(Some("<"), Box::new(ops::less)).into()),
                Token::LessEq  => Some(Function::new(Some("<="), Box::new(ops::less_eq)).into()),
                Token::Greater => Some(Function::new(Some(">"), Box::new(ops::greater)).into()),
                Token::GreaterEq => Some(Function::new(Some(">"), Box::new(ops::greater_eq)).into()),

                // Keywords
                Token::Not     => Some(Function::new(Some("not"), Box::new(ops::not)).into()),
                Token::Or      => Some(Function::new(Some("or"), Box::new(ops::or)).into()),
                Token::And     => Some(Function::new(Some("and"), Box::new(ops::and)).into()),
                Token::Print   => Some(Function::new(Some("print"), Box::new(ops::print)).into()),
                _ => None,
            }
        })
        .parse_stream(input)
}

fn list<I>(input: I) -> ParseResult<Expr, I>
    where I: Stream<Item = Token>
{
    let nil = token(Token::LParen).and(token(Token::RParen))
        .map(|_| Expr::from(Atom::Nil));
    try(between(token(Token::LParen),
            token(Token::RParen),
            many1(parser(expr)).map(List).map(Expr::List)))
        .or(nil)
        .parse_stream(input)
}

fn quote<I>(input: I) -> ParseResult<Expr, I>
    where I: Stream<Item = Token>
{
    between(token(Token::LBracket),
            token(Token::RBracket),
            many(parser(expr)).map(Quote).map(Expr::Quote))
        .parse_stream(input)
}

fn symbol<I>(input: I) -> ParseResult<Expr, I>
    where I: Stream<Item = Token>
{
    satisfy_map(|token| {
            if let Token::Symbol(sym) = token {
                Some(Expr::from(Symbol(sym)))
            } else {
                None
            }
        })
        .parse_stream(input)
}
