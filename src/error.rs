use combine;
use combine::primitives::IteratorStream;
use std::vec;
use token::Token;
// error_chain! {
//     errors {
//         Eof
//         Exit(status: i32)
//     }
// }

#[derive(Debug, error_chain)]
pub enum ErrorKind {
    Msg(String),

    #[error_chain(foreign)]
    Parse(combine::ParseError<combine::State<IteratorStream<vec::IntoIter<Token>>>>),

    #[error_chain(custom)]
    Eof,

    #[error_chain(custom)]
    Exit(i32),
}