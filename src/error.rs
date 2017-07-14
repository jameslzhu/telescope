use combine;
use combine::primitives::IteratorStream;
use std::io;
use std::vec;
use token::Token;
use types::{Symbol, Expr};

#[derive(Debug, error_chain)]
pub enum ErrorKind {
    Msg(String),

    #[error_chain(custom)]
    #[error_chain(description = r#"|_| "evaluation error""#)]
    #[error_chain(display = r#"|t| write!(f, "evaluation error: {}", t)"#)]
    Eval(Expr),

    #[error_chain(custom)]
    #[error_chain(description = r#"|_| "undefined symbol""#)]
    #[error_chain(display = r#"|t| write!(f, "undefined symbol {}", t)"#)]
    UndefinedSymbol(Symbol),

    #[error_chain(foreign)]
    Io(io::Error),

    #[error_chain(foreign)]
    Parse(combine::ParseError<combine::State<IteratorStream<vec::IntoIter<Token>>>>),

    #[error_chain(custom)]
    Eof,

    #[error_chain(custom)]
    Exit(i32),
}
