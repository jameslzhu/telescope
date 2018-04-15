// use std::fs;
use token::Token;
use std::io;
// use stream::{StringStream, TokenStream};
use combine;
use failure;

pub type Result<T> = ::std::result::Result<T, failure::Error>;

#[derive(Debug, Fail)]
pub enum Error<'a> {
    #[fail(display = "{}", _0)]
    Msg(String),

    // #[error_chain(custom)]
    // #[error_chain(description = r#"|_| "undefined symbol""#)]
    // #[error_chain(display = r#"|t| write!(f, "undefined symbol {}", t)"#)]
    // UndefinedSymbol(Symbol),

    #[fail(display = "IO error: {}", _0)]
    Io(io::Error),

    // #[error_chain(foreign)]
    #[fail(display = "Lex error: {}", _0)]
    Lex(combine::easy::Error<char, &'a str>),

    #[fail(display = "Parse error: {:?}", _0)]
    Parse(combine::easy::Error<Token, &'a [Token]>),

    // #[error_chain(custom)]
    #[fail(display = "Unexpected eof")]
    Eof,

    #[fail(display = "exit code {}", _0)]
    Exit(i32),

    // #[error_chain(custom)]
    // #[error_chain(description = r#"|_, _| "type error""#)]
    // #[error_chain(display = r#"|f, value, type| write!(f, "type error: received {}, expected {}", value, type)"#)]
    // Type(Expr, String),
}
