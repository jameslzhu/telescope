use std::fmt;
use combine::primitives::Positioner;
use conv::ValueFrom;

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Bool(bool),
    Int(i64),
    Flt(f64),
    Str(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    LBracket,
    RBracket,
    Literal(Literal),
    Symbol(String),
}

impl Positioner for Token {
    type Position = <char as Positioner>::Position;

    fn start() -> Self::Position {
        char::start()
    }

    fn update(&self, position: &mut Self::Position) {
        match *self {
            Token::LParen => position.column += 1,
            Token::RParen => position.column += 1,
            Token::LBracket => position.column += 1,
            Token::RBracket => position.column += 1,
            Token::Literal(ref l) => position.column += i32::value_from(l.to_string().len()).unwrap(),
            Token::Symbol(ref s) => position.column += i32::value_from(s.len()).unwrap(),
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Literal::Bool(ref b) => write!(f, "{}", b),
            Literal::Int(ref i) => write!(f, "{}", i),
            Literal::Flt(ref x) => write!(f, "{}", x),
            Literal::Str(ref s) => write!(f, "{}", s),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Literal(ref lit) => write!(f, "{}", lit),
            Token::Symbol(ref s) => write!(f, "{}", s),
            _ => write!(f, "{:#?}", self),
        }
    }
}

impl From<i32> for Literal {
    fn from(x: i32) -> Self {
        Literal::Int(x.into())
    }
}

impl From<i64> for Literal {
    fn from(x: i64) -> Self {
        Literal::Int(x)
    }
}

impl From<f32> for Literal {
    fn from(x: f32) -> Self {
        Literal::Flt(x.into())
    }
}

impl From<f64> for Literal {
    fn from(x: f64) -> Self {
        Literal::Flt(x)
    }
}

impl From<bool> for Literal {
    fn from(x: bool) -> Self {
        Literal::Bool(x)
    }
}

impl<'a> From<&'a str> for Literal {
    fn from(x: &'a str) -> Self {
        Literal::Str(x.to_owned())
    }
}

impl From<String> for Literal {
    fn from(x: String) -> Self {
        Literal::Str(x)
    }
}

impl<T> From<T> for Token
where
    T: Into<Literal>,
{
    fn from(x: T) -> Self {
        Token::Literal(x.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn format_literal() {
        let int = Literal::from(32i64);
        let flt = Literal::from(3.14f64);
        let boolean = Literal::from(true);
        let string = Literal::from("jkl;");

        assert_eq!("32", int.to_string());
        assert_eq!("3.14", flt.to_string());
        assert_eq!("true", boolean.to_string());
        assert_eq!("jkl;", string.to_string());
    }

    #[test]
    fn format_token() {
        let lparen = Token::LParen;

        // Literal
        let literal = Token::from(1);

        assert_eq!("LParen", lparen.to_string());
        assert_eq!("1", literal.to_string());
    }
}
