// mod error;
mod lexer;
mod parser;

use combine::{Parser};
use combine::easy;
use combine::stream;
use combine::parser::combinator::{AnyPartialState};

pub use error::Error;
use types::Expr;
// use token::Token;

const TEXT_BUF_CAPACITY: usize = 256;
const TOKEN_BUF_CAPACITY: usize = 256;

pub fn parse_source(src: &str) -> Result<Vec<Expr>, Error> {
    parse_line(src, None)
}

pub fn parse_line(src: &str, token_state: Option<&mut AnyPartialState>)
    -> Result<Vec<Expr>, Error>
{
    let lexer = lexer::lexer();
    let parser = parser::parser();

    let mut token_state = token_state.unwrap_or(&mut <AnyPartialState as Default>::default());

    let mut text_stream = easy::Stream(stream::PartialStream(src));
    lexer.parse_stream(&mut text_stream)
        .map_err(|_| Error::Lex)
        .and_then(|(tokens, consumed_data)| {
            let token_stream = easy::Stream(stream::PartialStream(tokens.as_slice()));
            stream::decode(parser, token_stream, &mut token_state)
                .map_err(|_| Error::Parse)
        })
        .map(|(exprs, _)| {
            match exprs {
                Some(exprs) => exprs,
                None => Vec::new()
            }
        })
}