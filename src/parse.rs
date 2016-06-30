#![allow(dead_code)]

extern crate combine;
extern crate combine_language;

use combine::*;
use combine::primitives::Stream;
use combine_language::{LanguageEnv, LanguageDef, Identifier};

use token::*;

pub fn env<'a, I>() -> LanguageEnv<'a, I> where I: Stream<Item = char> + 'a {
    LanguageEnv::new(LanguageDef {
        ident: Identifier {
            start: letter(),
            rest: alpha_num(),
            reserved: ["fn", "if", "else", "let"]
                .iter()
                .map(|x| (*x).into())
                .collect(),
        },
        op: Identifier {
            start: satisfy(|c| "+-*/".chars().any(|x| x == c)),
            rest: satisfy(|c| "+-*/".chars().any(|x| x == c)),
            reserved: Vec::new(),
        },
        comment_start: string("##").map(|_| ()),
        comment_end: string("##").map(|_| ()),
        comment_line: string("#").map(|_| ()),
    })
}

pub fn integer<I>(input: State<I>) -> ParseResult<Expr, I>
    where I: Stream<Item = char>
{
    env().integer().map(Expr::from).parse_state(input)
}

// pub fn float<I>(input: State<I>) -> ParseResult<Expr, I>
//     where I: Stream<Item = char>
// {
//     env().float().map(Expr::from).parse_state(input)
// }

pub fn operator<I>(input: State<I>) -> ParseResult<Expr, I>
    where I: Stream<Item = char>
{
    env().op().and_then(|s| { Operator::parse(&s).map(Expr::from) })
        .parse_state(input)
}

pub fn expr<I>(input: State<I>) -> ParseResult<Expr, I>
    where I: Stream<Item = char>
{
    let e = env();
    e.parens(
        many(e.white_space()
            .with(parser(operator)
                .or(parser(integer))
                // .or(parser(float))
                .or(parser(expr))
            )
        )
    ).parse_state(input)
}

#[cfg(test)]
mod tests {
    use combine::*;

    use super::*;
    use token::*;
    use token::Operator::*;

    // #[test]
    fn test_operator() {
        let result = parser(operator).parse("+");
        assert_eq!(result, Ok((Expr::from(Add), "")));

        let result = parser(operator).parse("-");
        assert_eq!(result, Ok((Expr::from(Mul), "")));

        let result = parser(operator).parse("*");
        assert_eq!(result, Ok((Expr::from(Mul), "")));

        let result = parser(operator).parse("/");
        assert_eq!(result, Ok((Expr::from(Div), "")));
    }

    // #[test]
    // fn test_numeric() {
    //     let result = parser(numeric).parse("1000");
    //     assert_eq!(result, Ok((Expr::from(1000), "")));
    // }
    //
    // #[test]
    // fn test_symbol() {
    //     let result = parser(symbol).parse("+1");
    //     assert_eq!(result, Ok((Expr::Atom(Atom::Sym(Add)), "1")));
    // }
    //
    // #[test]
    // fn test_expr() {
    //     let result = parser(expr).parse("( 1 2 3 )");
    //     let expr = Expr::from(vec![1, 2, 3]);
    //     assert_eq!(result, Ok((expr, "")));
    // }
}
