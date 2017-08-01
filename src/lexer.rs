use combine::{Parser, Stream, ParseError, ParseResult};
use combine::{between, many, many1, one_of, optional, parser, satisfy, satisfy_map, try};
use combine::char::{digit, char, spaces};

use token::{Literal, Token};
use unicode_xid::UnicodeXID;

pub fn lex<I>(input: I) -> Result<(Vec<Token>, I), ParseError<I>>
where
    I: Stream<Item = char>,
{
    between(spaces(),
            spaces(),
            many(spaces().with(parser(token))))
        .parse(input)
}

fn token<I>(input: I) -> ParseResult<Token, I>
where
    I: Stream<Item = char>,
{
    parser(symbol)
        .or(parser(literal))
        .or(parser(punctuation))
        .parse_stream(input)
}

fn literal<I>(input: I) -> ParseResult<Token, I>
where
    I: Stream<Item = char>,
{
    let sign = optional(char('-'));
    let digits = many1::<String, _>(digit());
    let integer =
    (
        sign.clone(),
        digits.clone().and_then(|s| s.parse::<i64>())
    )
        .map(|(sign, num)| if sign.is_some() { -num } else { num })
        .map(Literal::from);

    let float = sign.clone()
        .and(
            (digits.clone(), char('.'), digits.clone())
                .map(|(prefix, _, suffix)| format!("{}.{}", prefix, suffix))
                .and_then(|num| num.parse::<f64>()),
        )
        .map(|(sign, num)| if sign.is_some() { -num } else { num })
        .map(Literal::from);

    let num = try(float).or(try(integer));

    let boolean = char('#')
        .with(satisfy_map(|c| match c {
            't' => Some(true),
            'f' => Some(false),
            _ => None,
        }))
        .map(Literal::from);

    let escaped = char('\\').with(satisfy_map(|c| match c {
        '\"' => Some('\"'),
        '\\' => Some('\\'),
        'n' => Some('\n'),
        'r' => Some('\r'),
        't' => Some('\t'),
        _ => None,
    }));

    let non_quote = try(escaped).or(satisfy(|c| c != '"'));

    let string = between(char('"'), char('"'), many::<String, _>(non_quote)).map(Literal::from);

    boolean
        .or(num)
        .or(string)
        .map(Token::from)
        .parse_stream(input)
}

fn symbol<I>(input: I) -> ParseResult<Token, I>
where
    I: Stream<Item = char>,
{
    let punctuation = one_of("_+-*/=<>!".chars());
    let start = satisfy(UnicodeXID::is_xid_start).or(punctuation.clone());
    let body = satisfy(UnicodeXID::is_xid_continue).or(punctuation.clone());
    let rest = many::<String, _>(body);
    start
        .and(rest)
        .map(|(f, mut r)| {
            r.insert(0, f);
            r
        })
        .map(Token::Symbol)
        .parse_stream(input)
}

fn punctuation<I>(input: I) -> ParseResult<Token, I>
where
    I: Stream<Item = char>,
{
    satisfy_map(|c| match c {
        '(' => Some(Token::LParen),
        ')' => Some(Token::RParen),
        '[' => Some(Token::LBracket),
        ']' => Some(Token::RBracket),
        '\'' => Some(Token::Quote),
        _ => None,
    }).parse_stream(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::ApproxEqUlps;

    #[test]
    fn parse_bool_literal() {
        assert_eq!(Ok((Token::from(true), "")), parser(literal).parse("#t"));
        assert_eq!(Ok((Token::from(false), "")), parser(literal).parse("#f"));
    }

    #[test]
    fn parse_zero_literal() {
        // Integer case
        assert_eq!(Ok((Token::from(0i64), "")), parser(literal).parse("0"));

        // Float case
        match parser(literal).parse("0.0") {
            Ok((Token::Literal(Literal::Flt(flt)), _)) => assert!(flt.approx_eq_ulps(&0.0f64, 4)),
            Ok(x) => assert!(false, format!("0.0 parsed as {}", x.0)),
            Err(e) => assert!(false, format!("0.0 parsed as {}", e)),
        };
    }

    #[test]
    fn parse_escape_chars() {
        assert_eq!(
            Ok((Token::from("\""), "")),
            parser(literal).parse(r#""\"""#)
        );
        assert_eq!(
            Ok((Token::from("\\"), "")),
            parser(literal).parse(r#""\\""#)
        );
        assert_eq!(
            Ok((Token::from("\n"), "")),
            parser(literal).parse(r#""\n""#)
        );
        assert_eq!(
            Ok((Token::from("\r"), "")),
            parser(literal).parse(r#""\r""#)
        );
        assert_eq!(
            Ok((Token::from("\t"), "")),
            parser(literal).parse(r#""\t""#)
        );
    }

    #[test]
    fn parse_empty() {
        assert_eq!(Ok((Vec::new(), "")), lex(""));
    }

    quickcheck!{
        fn parse_int_literal(x: i64) -> bool {
            Ok((Token::from(x), "")) == parser(literal).parse(&*x.to_string())
        }

        fn parse_float_literal(x: f64) -> bool {
            let mut string = x.to_string();
            if x.trunc() == x {
                string.push_str(".0");
            }

            if let Ok((Token::Literal(Literal::Flt(y)), _)) = parser(literal).parse(&*string) {
                x.approx_eq_ulps(&y, 4)
            } else {
                false
            }
        }

        fn parse_str_literal(x: String) -> bool {
            let string = format!("\"{}\"", x
                .replace("\\", r"\\")
                .replace("\n", r"\n")
                .replace("\r", r"\r")
                .replace("\t", r"\t")
                .replace("\"", r#"\""#));
            Ok((Token::from(x.clone()), "")) == parser(literal).parse(&*string)
        }
    }
}
