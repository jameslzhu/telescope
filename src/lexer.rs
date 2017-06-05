#![allow(dead_code)]

use combine::{between, choice, many, many1, optional, parser, satisfy, satisfy_map, sep_by, try};
use combine::{Parser, Stream, ParseError, ParseResult};
use combine::char::{alpha_num, digit, char, letter, spaces, string};

use token::{Literal, Token};

pub fn lex<'a>(line: &'a str) -> Result<(Vec<Token>, &'a str), ParseError<&'a str>> {
    spaces().with(parser(token_stream))
        .parse(line)
}

fn literal<I>(input: I) -> ParseResult<Literal, I>
    where I: Stream<Item=char>
{
    let digits = many1::<String, _>(digit());
    let integer = optional(char('-'))
        .and(digits.clone())
        .map(|(sign, s)| {
            let magnitude = s.parse::<i64>().unwrap();
            if sign.is_some() { -magnitude } else { magnitude }
        })
        .map(Literal::from);
    
    let float = optional(char('-'))
        .and((digits.clone(), char('.'), digits.clone())
            .map(|(prefix, _, suffix)| format!("{}.{}", prefix, suffix)))
        .map(|(sign, s)| {
            let magnitude = s.parse::<f64>().unwrap();
            if sign.is_some() { -magnitude } else { magnitude }
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
    
    let non_quote = satisfy(|c| c != '"');
    
    let string = between(char('"'), char('"'), many::<String, _>(non_quote))
        .map(Literal::from);
    
    boolean.or(string).or(num)
        .parse_stream(input)
}

fn symbol<I>(input: I) -> ParseResult<Token, I>
    where I: Stream<Item=char>
{
    let first = letter().or(char('_'));
    let rest = many::<String, _>(alpha_num().or(char('_')));
    first.and(rest)
        .map(|(f, mut r)| { r.insert(0, f); r })
        .map(Token::Symbol)
        .parse_stream(input)
}

fn punctuation<I>(input: I) -> ParseResult<Token, I>
    where I: Stream<Item=char>
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
            _ => None
        }
    });

    let two_char_tokens = choice([
        string("<="),
        string(">="),
    ]).map(|s| match s {
        "<=" => Token::LessEqual,
        ">=" => Token::GreaterEqual,
        _ => unreachable!(),
    });

    // Maximal munch: attempt parsing two-char tokens before one-char tokens
    two_char_tokens.or(single_char_tokens).parse_stream(input)
}

fn token<I>(input: I) -> ParseResult<Token, I>
    where I: Stream<Item=char>
{
    parser(symbol)
        .or(parser(literal).map(Token::Literal))
        .or(parser(symbol))
        .or(parser(punctuation))
        .parse_stream(input)
}

fn token_stream<I>(input: I) -> ParseResult<Vec<Token>, I>
    where I: Stream<Item=char>
{
    sep_by(parser(token), spaces()).parse_stream(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::ApproxEqUlps;
    use quickcheck::TestResult;

    #[test]
    fn parse_bool_literal() {
        assert_eq!(Ok((Literal::Bool(true), "")), parser(literal).parse("#t"));
        assert_eq!(Ok((Literal::Bool(false), "")), parser(literal).parse("#f"));
    }

    #[test]
    fn parse_zero_literal() {
        // Integer case
        assert_eq!(Ok((Literal::Int(0i64), "")), parser(literal).parse("0"));

        // Float case
        match parser(literal).parse("0.0") {
            Ok((Literal::Flt(flt), _)) => assert!(flt.approx_eq_ulps(&0.0f64, 4)),
            Ok(x) => assert!(false, format!("0.0 parsed as {}", x.0)),
            Err(e) => assert!(false, format!("0.0 parsed as {}", e)),
        };
    }

    #[test]
    fn parse_empty() {
        assert_eq!(Ok((Vec::new(), "")), parser(token_stream).parse(""));
    }

    quickcheck!{
        fn parse_int_literal(x: i64) -> bool {
            match parser(literal).parse(&*x.to_string()) {
                Ok((Literal::Int(y), _)) => x == y,
                _ => false,
            }
        }

        fn parse_float_literal(x: f64) -> bool {
            let mut string = x.to_string();
            if x.trunc() == x {
                string.push_str(".0");
            }

            if let Ok((Literal::Flt(y), _)) = parser(literal).parse(&*string) {
                x.approx_eq_ulps(&y, 4)
            } else {
                false
            }
        }

        fn parse_str_literal(x: String) -> TestResult {
            let string = format!("\"{}\"", x);
            if x.contains('\"') {
                return TestResult::discard()
            }
            match parser(literal).parse(&*string) {
                Ok((Literal::Str(result), _)) => TestResult::from_bool(result == x),
                _ => TestResult::failed(),
            }
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
