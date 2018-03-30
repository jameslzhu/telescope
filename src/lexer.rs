use std::num;

use combine::{Parser, Stream, ParseError};
use combine::{between, many, many1, one_of, optional, satisfy, satisfy_map, try};
use combine::char::{digit, char, spaces};

use token::{Literal, Token};
use unicode_xid::UnicodeXID;

pub fn lex<I>(input: I) -> Result<(Vec<Token>, I), I::Error>
where
    I: Stream<Item = char>,
    I::Error: ParseError<char, I::Range, I::Position>,
    <I::Error as ParseError<I::Item, I::Range, I::Position>>::StreamError: From<num::ParseIntError>,
    <I::Error as ParseError<I::Item, I::Range, I::Position>>::StreamError: From<num::ParseFloatError>,
{
    parser!{
        fn lexer[I]()(I) -> Vec<Token>
            where [
                I: Stream<Item=char>, 
                I::Error: ParseError<char, I::Range, I::Position>,
                <I::Error as ParseError<I::Item, I::Range, I::Position>>::StreamError: From<num::ParseIntError>,
                <I::Error as ParseError<I::Item, I::Range, I::Position>>::StreamError: From<num::ParseFloatError>,
            ]
        {
        between(spaces(),
                spaces(),
                many(spaces().with(token())))
        }
    }

    lexer().parse(input)
}

parser!{
    fn token[I]()(I) -> Token
    where [
        I: Stream<Item = char>,
        I::Error: ParseError<char, I::Range, I::Position>,
        <I::Error as ParseError<I::Item, I::Range, I::Position>>::StreamError: From<num::ParseIntError>,
        <I::Error as ParseError<I::Item, I::Range, I::Position>>::StreamError: From<num::ParseFloatError>,
    ]
    {
        symbol()
            .or(literal())
            .or(punctuation())
    }
}

parser!{
    fn literal[I]()(I) -> Token
    where [
        I: Stream<Item = char>,
        I::Error: ParseError<char, I::Range, I::Position>,
        <I::Error as ParseError<I::Item, I::Range, I::Position>>::StreamError : From<num::ParseIntError>,
        <I::Error as ParseError<I::Item, I::Range, I::Position>>::StreamError : From<num::ParseFloatError>,
    ]
    {
        let sign = optional(char('-'));
        let digits = many1::<String, _>(digit());
        let integer =
        (
            sign,
            digits.and_then(|s| s.parse::<i64>())
        )
            .map(|(sign, num)| if sign.is_some() { -num } else { num })
            .map(Literal::from);

        let float = sign
            .and(
                (digits, char('.'), digits)
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
    }
}

parser!{
    fn symbol[I]()(I) -> Token
    where [I: Stream<Item = char>]
    {
        let punctuation = one_of("_+-*/=<>!".chars());
        let start = satisfy(UnicodeXID::is_xid_start).or(punctuation);
        let body = satisfy(UnicodeXID::is_xid_continue).or(punctuation);
        let rest = many::<String, _>(body);
        start
            .and(rest)
            .map(|(f, mut r)| {
                r.insert(0, f);
                r
            })
            .map(Token::Symbol)
    }
}

parser!{
    fn punctuation[I]()(I) -> Token
        where [I: Stream<Item=char>]
    {
        satisfy_map(|c| match c {
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            '[' => Some(Token::LBracket),
            ']' => Some(Token::RBracket),
            '\'' => Some(Token::Quote),
            _ => None,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::ApproxEqUlps;

    #[test]
    fn empty() {
        assert_eq!(Ok((vec![], "")), lex(""));
    }

    #[test]
    fn bool_literal() {
        assert_eq!(Ok((Token::from(true), "")), parser(literal).parse("#t"));
        assert_eq!(Ok((Token::from(false), "")), parser(literal).parse("#f"));
    }

    #[test]
    fn zero_literal() {
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
    fn escape_chars() {
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
    fn delimiters() {
        // Single-character tests
        assert_eq!(
            Ok((Token::LParen, "")),
            parser(punctuation).parse("(")
        );

        assert_eq!(
            Ok((Token::RParen, "")),
            parser(punctuation).parse(")")
        );

        assert_eq!(
            Ok((Token::LBracket, "")),
            parser(punctuation).parse("[")
        );

        assert_eq!(
            Ok((Token::RBracket, "")),
            parser(punctuation).parse("]")
        );
    }

    #[test]
    fn nested_lists() {
        assert_eq!(
            Ok((vec![Token::LParen, Token::LParen, Token::RParen, Token::RParen],"")),
            lex("(())")
        );
    }

    #[test]
    fn deep_nested_lists() {
        use std::iter;
        let num_layers = 1000;
        let input = ["(".repeat(num_layers), ")".repeat(num_layers)].concat();
        let left = iter::repeat(Token::LParen).take(num_layers);
        let right = iter::repeat(Token::RParen).take(num_layers);
        let output = left.chain(right).collect::<Vec<_>>();
        assert_eq!(
            Ok((output, "")),
            lex(&*input)
        );
    }

    quickcheck!{
        fn int_literal(x: i64) -> bool {
            Ok((Token::from(x), "")) == parser(literal).parse(&*x.to_string())
        }

        fn float_literal(x: f64) -> bool {
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

        fn str_literal(x: String) -> bool {
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
