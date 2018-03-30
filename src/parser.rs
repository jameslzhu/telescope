use combine::{Stream, Parser, ParseError};
use combine::{between, many, satisfy_map, token, try, not_followed_by};
use combine::error::Info;
use token::Token;
use types::{Expr, List, Vector, Symbol};

pub fn parse<I>(input: I) -> Result<(Vec<Expr>, I), I::Error>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<Token, I::Range, I::Position>,
    Info<Token, I::Range>: From<Token>,
{
    // Balanced delimiters
    many(expr())
        .skip(not_followed_by(token(Token::RParen)))
        .skip(not_followed_by(token(Token::RBracket)))
        .parse(input)
}

parser!{
    fn expr[I]()(I) -> Expr
    where [
        I: Stream<Item = Token>,
    ]
    {
        choice!(
            atom,
            quote,
            list,
            vector
        )
    }
}

parser!{
    fn quote[I]()(I) -> Expr
    where [
        I: Stream<Item = Token>,
    ]
    {
        (
            token(Token::Quote),
            expr()
        )
        .map(|(_, e)| {
            let quote_symbol = Expr::Sym(Symbol("quote".into()));
            Expr::List(List(vec![quote_symbol, e]))
        })
    }
}

parser!{
    fn atom[I]()(I) -> Expr
    where [
        I: Stream<Item = Token>,
    ]
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
        })
    }
}

parser!{
    fn list[I]()(I) -> Expr
    where [
        I: Stream<Item = Token>,
    ]
    {
            try(between(
                token(Token::LParen),
                token(Token::RParen),
                many(expr()).map(List).map(Expr::List),
            ))
    }
}

parser!{
    fn vector[I]()(I) -> Expr
    where [
        I: Stream<Item = Token>,
    ]
    {
        try(between(
            token(Token::LBracket),
            token(Token::RBracket),
            many(expr()).map(Vector).map(Expr::Vector),
        ))
    }
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