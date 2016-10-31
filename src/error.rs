use std::fmt;
use lalrpop_util::ParseError;

error_chain! {
    links {}
    foreign_links {}
    errors {
        Parse(t: String) {
            description("parsing error")
            display("parsing error: {}", t)
        }
    }
}

impl<L,T> From<ParseError<L,T,Error>> for Error
    where L: Into<usize>, T: fmt::Debug
{
    fn from(e: ParseError<L,T,Error>) -> Self {
        match e {
            ParseError::InvalidToken{ location } => {
                ErrorKind::Parse(format!("invalid token at {}", location.into())).into()
            },
            ParseError::UnrecognizedToken { token, expected } => {
                ErrorKind::Parse(format!(
                    "unrecognized token {}, expected ({})",
                    token.map(|(l, t, r)| format!("{:?} at [{}:{}]", t, l.into(), r.into()))
                         .unwrap_or(String::new()),
                    expected.join(", "))).into()
            },
            ParseError::ExtraToken { token: (l, t, r) } => {
                ErrorKind::Parse(format!("extra token {:?} at [{}:{}]", t, l.into(), r.into())).into()
            },
            ParseError::User { error } => error,
        }
    }
}