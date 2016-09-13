extern crate lrs;

#[cfg(test)]
mod tests {
    use lrs::atom::*;
    use lrs::token::*;
    use lrs::token::Operator::*;

    // #[test]
    fn test_operator() {
        let result = parse_Op("+");
        assert_eq!(result, Ok(Atom::from(Add)));

        let result = parse_Op("-");
        assert_eq!(result, Ok(Atom::from(Sub)));

        let result = parse_Op("*");
        assert_eq!(result, Ok(Atom::from(Mul)));

        let result = parse_Op("/");
        assert_eq!(result, Ok(Atom::from(Div)));
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
