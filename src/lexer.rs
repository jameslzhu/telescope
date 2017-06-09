use combine::{Parser, Stream, ParseError, ParseResult};
use combine::{between, choice, many, many1, optional, parser, satisfy, satisfy_map, sep_by, try};
use combine::char::{alpha_num, digit, char, letter, spaces, string};

use token::{Literal, Token};

pub fn lex<I>(input: I) -> Result<(Vec<Token>, I), ParseError<I>>
    where I: Stream<Item = char>
{
    spaces().with(sep_by(parser(token), spaces()))
        .parse(input)
}

fn literal<I>(input: I) -> ParseResult<Token, I>
    where I: Stream<Item = char>
{
    let sign = optional(char('-'));
    let digits = many1::<String, _>(digit());
    let integer = sign.clone()
        .and(digits.clone()
                .and_then(|s| s.parse::<i64>()))
        .map(|(sign, num)| {
            if sign.is_some() { -num } else { num }
        })
        .map(Literal::from);

    let float = sign.clone()
        .and((digits.clone(), char('.'), digits.clone())
            .map(|(prefix, _, suffix)| format!("{}.{}", prefix, suffix))
            .and_then(|num| num.parse::<f64>()))
        .map(|(sign, num)| {
            if sign.is_some() { -num } else { num }
        })
        .map(Literal::from);

    let num = try(float).or(try(integer));

    let boolean = char('#')
        .with(satisfy_map(|c| {
            match c {
                't' => Some(true),
                'f' => Some(false),
                _ => None,
            }
        }))
        .map(Literal::from);

    let escaped = char('\\')
        .with(satisfy_map(|c| {
            match c {
                '\"' => Some('\"'),
                '\\' => Some('\\'),
                'n' => Some('\n'),
                'r' => Some('\r'),
                't' => Some('\t'),
                _ => None,
            }
        }));

    let non_quote = try(escaped).or(satisfy(|c| c != '"'));

    let string = between(char('"'), char('"'), many::<String, _>(non_quote)).map(Literal::from);

    choice!(boolean, string, num)
        .map(Token::from)
        .parse_stream(input)
}

fn symbol<I>(input: I) -> ParseResult<Token, I>
    where I: Stream<Item = char>
{
    let first = letter().or(char('_'));
    let rest = many::<String, _>(alpha_num().or(char('_')));
    first.and(rest)
        .map(|(f, mut r)| {
            r.insert(0, f);
            r
        })
        .map(Token::Symbol)
        .parse_stream(input)
}

fn punctuation<I>(input: I) -> ParseResult<Token, I>
    where I: Stream<Item = char>
{
    let single_char_tokens = satisfy_map(|c| {
        match c {
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            '[' => Some(Token::LBracket),
            ']' => Some(Token::RBracket),
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Star),
            '/' => Some(Token::Slash),
            '=' => Some(Token::Equal),
            '<' => Some(Token::Less),
            '>' => Some(Token::Greater),
            _ => None,
        }
    });

    let two_char_tokens = choice([string("<="), string(">=")]).map(|s| match s {
        "<=" => Token::LessEq,
        ">=" => Token::GreaterEq,
        _ => unreachable!(),
    });

    // Maximal munch: attempt parsing two-char tokens before one-char tokens
    two_char_tokens.or(single_char_tokens).parse_stream(input)
}

fn token<I>(input: I) -> ParseResult<Token, I>
    where I: Stream<Item = char>
{
    choice!(
        parser(symbol),
        parser(literal),
        parser(symbol),
        parser(punctuation)
    ).parse_stream(input)
}

#[cfg(test)]
mod test {
    use float_cmp::ApproxEqUlps;
    use quickcheck::TestResult;
    use super::*;

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
        assert_eq!(Ok((Token::from("\""), "")), parser(literal).parse(r#""\"""#));
        assert_eq!(Ok((Token::from("\\"), "")), parser(literal).parse(r#""\\""#));
        assert_eq!(Ok((Token::from("\n"), "")), parser(literal).parse(r#""\n""#));
        assert_eq!(Ok((Token::from("\r"), "")), parser(literal).parse(r#""\r""#));
        assert_eq!(Ok((Token::from("\t"), "")), parser(literal).parse(r#""\t""#));
    }

    #[test]
    fn parse_empty() {
        assert_eq!(Ok((Vec::new(), "")), lex(""));
    }

    quickcheck!{
        fn parse_int_literal(x: i64) -> () {
            assert_eq!(Ok((Token::from(x), "")), parser(literal).parse(&*x.to_string()));
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
            let has_escape_chars = x.contains('\\') || x.contains('\"');
            let string = if has_escape_chars {
                x.chars()
                    .map(char::escape_default)
                    .map(|y| y.to_string())
                    .collect()
            } else {
                format!("\"{}\"", x)
            };
            Ok((Token::from(x.clone()), "")) == parser(literal).parse(&*string)
        }

        fn parse_symbol(x: String) -> TestResult {
            if x.is_empty() {
                return TestResult::discard()
            }

            if !x.chars().next().unwrap().is_alphabetic() {
                return TestResult::discard()
            }

            // Test only valid symbols
            for c in x.chars() {
                if !(c.is_alphanumeric() || c == '_') {
                    return TestResult::discard()
                }
            }

            match parser(symbol).parse(&*x) {
                Ok((Token::Symbol(result), _)) => TestResult::from_bool(result == x),
                _ => TestResult::failed(),
            }
        }
    }
}
