use combine::easy::{Stream, ParseError};
// use std::fs;
use std::io;
// use stream::{StringStream, TokenStream};
use failure;

pub type Result<T> = ::std::result::Result<T, failure::Error>;

#[derive(Debug, Fail)]
pub enum Error {
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
    Lex(ParseError<Stream<String>>),

    #[fail(display = "Parse error: {}", _0)]
    Parse(ParseError<Stream<Token>>),

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
