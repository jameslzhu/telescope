//use std::fmt;
use combine::{StreamOnce, ParseError};
use combine::primitives::SourcePosition;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }
    links {}
    foreign_links {}
    errors {
        Parse(position: i32, t: String) {
            description("parsing error")
            display("parsing error at {}: {}", position, t)
        }
    }
}

impl<S: StreamOnce> From<ParseError<S>> for Error
    where S: StreamOnce<Position = SourcePosition>
{
    fn from(e: ParseError<S>) -> Self {
        Error::from_kind(ErrorKind::Parse(e.position.column, String::new()))
    }
}

/*
impl<L, T> From<ParseError<L, T, Error>> for Error
    where L: Into<usize>,
          T: fmt::Debug
{
    fn from(e: ParseError<L, T, Error>) -> Self {
        match e {
            ParseError::InvalidToken { location } => {
                ErrorKind::Parse(format!("invalid token at {}", location.into())).into()
            }
            ParseError::UnrecognizedToken { token, expected } => {
                ErrorKind::Parse(format!("unrecognized token {}, expected ({})",
                                         token.map(|(l, t, r)| {
                                                 format!("{:?} at [{}:{}]", t, l.into(), r.into())
                                             })
                                             .unwrap_or(String::new()),
                                         expected.join(", ")))
                    .into()
            }
            ParseError::ExtraToken { token: (l, t, r) } => {
                ErrorKind::Parse(format!("extra token {:?} at [{}:{}]", t, l.into(), r.into()))
                    .into()
            }
            ParseError::User { error } => error,
        }
    }
}
*/
