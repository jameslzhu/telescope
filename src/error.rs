use combine;
use combine::primitives::IteratorStream;
use std::vec;
use std::io;
use token::Token;

#[derive(Debug, error_chain)]
pub enum ErrorKind {
    Msg(String),

    #[error_chain(foreign)]
    Io(io::Error),

    #[error_chain(foreign)]
    Parse(combine::ParseError<combine::State<IteratorStream<vec::IntoIter<Token>>>>),

    #[error_chain(custom)]
    Eof,

    #[error_chain(custom)]
    Exit(i32),
}
