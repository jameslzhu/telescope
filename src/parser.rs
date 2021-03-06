use combine::{Stream, Parser, ParseError, ParseResult};
use combine::{between, many, parser, satisfy_map, token, try, not_followed_by};
use token::Token;
use types::{Expr, List, Vector, Symbol};

pub fn parse<I>(input: I) -> Result<(Vec<Expr>, I), ParseError<I>>
where
    I: Stream<Item = Token>,
{
    // Balanced delimiters
    many(parser(expr))
        .skip(not_followed_by(token(Token::RParen)))
        .skip(not_followed_by(token(Token::RBracket)))
        .parse(input)
}

fn expr<I>(input: I) -> ParseResult<Expr, I>
where
    I: Stream<Item = Token>,
{
    choice!(
        parser(atom),
        parser(quote),
        parser(list),
        parser(vector)
    ).parse_stream(input)
}

fn quote<I>(input: I) -> ParseResult<Expr, I>
where
    I: Stream<Item = Token>,
{
    (
        token(Token::Quote),
        parser(expr)
    )
    .map(|(_, expr)| {
        let quote_symbol = Expr::Sym(Symbol("quote".into()));
        Expr::List(List(vec![quote_symbol, expr]))
    }).parse_stream(input)
}

fn atom<I>(input: I) -> ParseResult<Expr, I>
where
    I: Stream<Item = Token>,
{
    satisfy_map(|token| match token {
        Token::Literal(lit) => Some(Expr::from(lit)),
        Token::Symbol(sym) => {
            if sym == "nil" {
                Some(Expr::Nil)
            } else {
                Some(Expr::from(Symbol(sym)))
            }
        },
        _ => None,
    }).parse_stream(input)
}

fn list<I>(input: I) -> ParseResult<Expr, I>
where
    I: Stream<Item = Token>,
{
        try(between(
            token(Token::LParen),
            token(Token::RParen),
            many(parser(expr)).map(List).map(Expr::List),
        ))
        .parse_stream(input)
}

fn vector<I>(input: I) -> ParseResult<Expr, I>
where
    I: Stream<Item = Token>,
{
    try(between(
        token(Token::LBracket),
        token(Token::RBracket),
        many(parser(expr)).map(Vector).map(Expr::Vector),
    ))
    .parse_stream(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_list() {
        let input = vec![Token::LParen, Token::RParen];
        let output = vec![Expr::List(List(Vec::new()))];
        let empty: &[Token] = &[];
        assert_eq!(
            Ok((output, empty)),
            parse(&*input)
        );
    }

    #[test]
    fn empty_vector() {
        let input = vec![Token::LBracket, Token::RBracket];
        let output = vec![Expr::Vector(Vector(Vec::new()))];
        let empty: &[Token] = &[];
        assert_eq!(
            Ok((output, empty)),
            parse(&*input)
        );
    }
}