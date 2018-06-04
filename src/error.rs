// use std::fs;
// use token::Token;
use std::io;
// use stream::{StringStream, TokenStream};
// use combine::easy;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "{}", _0)]
    Msg(String),

    // #[error_chain(custom)]
    // #[error_chain(description = r#"|_| "undefined symbol""#)]
    // #[error_chain(display = r#"|t| write!(f, "undefined symbol {}", t)"#)]
    // UndefinedSymbol(Symbol),

    #[fail(display = "IO error: {}", _0)]
    Io(#[cause] io::Error),

    #[fail(display = "Lex error: {}", _0)]
    Lex(String),

    #[fail(display = "Parse error")]
    Parse,

    #[fail(display = "Unexpected eof")]
    Eof,

    #[fail(display = "exit code {}", _0)]
    Exit(i32),

    // #[error_chain(custom)]
    // #[error_chain(description = r#"|_, _| "type error""#)]
    // #[error_chain(display = r#"|f, value, type| write!(f, "type error: received {}, expected {}", value, type)"#)]
    // Type(Expr, String),
}
