use ops;
use token::Token;
use ast::{Expr, Function, List, Quote, Symbol};
use combine::{between, many, parser, satisfy_map, token};
use combine::{Stream, Parser, ParseError, ParseResult};

pub fn parse<I>(input: I) -> Result<(Vec<Expr>, I), ParseError<I>>
    where I: Stream<Item=Token>
{
    many(parser(expr)).parse(input)
}

fn expr<I>(input: I) -> ParseResult<Expr, I>
    where I: Stream<Item=Token>
{
    parser(atom)
        .or(parser(builtin))
        .or(parser(symbol))
        .or(parser(list))
        .or(parser(quote))
        .parse_stream(input)
}

fn atom<I>(input: I) -> ParseResult<Expr, I>
    where I: Stream<Item=Token>
{
    satisfy_map(|token| {
        if let Token::Literal(lit) = token {
            Some(Expr::from(lit))
        } else {
            None
        }
    }).parse_stream(input)
}

fn builtin<I>(input: I) -> ParseResult<Expr, I>
    where I: Stream<Item=Token>
{
    satisfy_map(|token| match token {
        // Math tokens
        Token::Plus     => Some(Function::new(Some("+"), Box::new(ops::add)).into()),
        Token::Minus    => Some(Function::new(Some("-"), Box::new(ops::sub)).into()),
        Token::Star     => Some(Function::new(Some("*"), Box::new(ops::mul)).into()),
        Token::Slash    => Some(Function::new(Some("/"), Box::new(ops::div)).into()),

        // Comparison tokens
        Token::Equal    => Some(Function::new(Some("="), Box::new(ops::equal)).into()),
        Token::Less     => Some(Function::new(Some("<"), Box::new(ops::less)).into()),
        Token::LessEq   => Some(Function::new(Some("<="), Box::new(ops::less_eq)).into()),
        Token::Greater  => Some(Function::new(Some(">"), Box::new(ops::greater)).into()),
        Token::GreaterEq => Some(Function::new(Some(">="), Box::new(ops::greater_eq)).into()),

        // Keywords
        Token::Not      => Some(Function::new(Some("not"), Box::new(ops::not)).into()),
        Token::Or       => Some(Function::new(Some("or"), Box::new(ops::or)).into()),
        Token::And      => Some(Function::new(Some("and"), Box::new(ops::and)).into()),
        Token::Print    => Some(Function::new(Some("print"), Box::new(ops::print)).into()),
        _ => None
    }).parse_stream(input)
}

fn list<I>(input: I) -> ParseResult<Expr, I>
    where I: Stream<Item=Token>
{
    between(token(Token::LParen),
            token(Token::RParen),
            many(parser(expr)).map(List).map(Expr::List))
        .parse_stream(input)
}

fn quote<I>(input: I) -> ParseResult<Expr, I>
    where I: Stream<Item=Token>
{
    between(token(Token::LBracket),
            token(Token::RBracket),
            many(parser(expr)).map(Quote).map(Expr::Quote))
        .parse_stream(input)
}

fn symbol<I>(input: I) -> ParseResult<Expr, I>
    where I: Stream<Item=Token>
{
    satisfy_map(|token| {
        if let Token::Symbol(sym) = token {
            Some(Expr::from(Symbol(sym)))
        } else {
            None
        }
    }).parse_stream(input)
}
