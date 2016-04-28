extern crate combine;

use combine::*;
use combine::primitives::Stream;

use atom::*;
use atom::Sym::*;

pub fn numeric<I>(input: State<I>) -> ParseResult<Expr<i32>, I>
    where I: Stream<Item = char>
{
    (// Sign parser
     optional(choice([token('+'), token('-')])),

     // Digits parser
     many1(digit()).map(|string: String| string.parse::<i32>().unwrap()))
        .map(|(sign, digits)| {
            Expr::from(if sign == Some('-') { -digits } else { digits })
        })
        .parse_state(input)
}

pub fn symbol<I>(input: State<I>) -> ParseResult<Expr<i32>, I>
    where I: Stream<Item = char>
{
    choice([token('+'), token('-'), token('*'), token('/')])
        .map(|sym| {
            Expr::Atom(Atom::Sym(match sym {
                '+' => Add,
                '-' => Sub,
                '*' => Mul,
                '/' => Div,
                _ => unreachable!(),
            }))
        })
        .parse_state(input)
}

pub fn expr<I>(input: State<I>) -> ParseResult<Expr<i32>, I>
    where I: Stream<Item = char>
{
    between(token('(').skip(spaces()),
            token(')'),
            many(parser(expr)
                .or(parser(symbol))
                .or(parser(numeric))
                .skip(spaces())
            )
        )
        .parse_state(input)
}

#[cfg(test)]
mod tests {
    use combine::*;

    use super::*;
    use atom::*;
    use atom::Sym::*;

    #[test]
    fn test_numeric() {
        let result = parser(numeric).parse("1000");
        assert_eq!(result, Ok((Expr::from(1000), "")));
    }

    #[test]
    fn test_symbol() {
        let result = parser(symbol).parse("+1");
        assert_eq!(result, Ok((Expr::Atom(Atom::Sym(Add)), "1")));
    }

    #[test]
    fn test_expr() {
        let result = parser(expr).parse("( 1 2 3 )");
        let expr = Expr::from(vec![1, 2, 3]);
        assert_eq!(result, Ok((expr, "")));
    }
}
