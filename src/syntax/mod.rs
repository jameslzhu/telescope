// mod error;
mod lexer;
mod parser;
mod token;

use combine::{Parser};
use combine::easy;
use combine::stream;
use combine::parser::combinator::{AnyPartialState};

pub use error::Error;
pub use syntax::token::Literal;
use types::Expr;
use syntax::token::Token;

// const TEXT_BUF_CAPACITY: usize = 256;
// const TOKEN_BUF_CAPACITY: usize = 256;

pub fn empty_state() -> AnyPartialState {
    <AnyPartialState as Default>::default()
}

pub fn parse_source(src: &str) -> Result<Vec<Expr>, Error> {
    let mut token_state = empty_state();
    let mut token_buffer = Vec::new();
    parse_line(src, &mut token_buffer, &mut token_state)
}

pub fn parse_line(src: &str, token_buffer: &mut Vec<Token>, mut token_state: &mut AnyPartialState)
    -> Result<Vec<Expr>, Error>
{
    let mut lexer = lexer::lexer();
    let parser = parser::parser();

    let mut text_stream = src;
    let (tokens, _consumed_data) = lexer.easy_parse(text_stream)
        .map_err(|e| Error::Lex(e.to_string()))?;
    token_buffer.extend(tokens);
    let token_stream = easy::Stream(stream::PartialStream(token_buffer.as_slice()));
    let (exprs, _) = stream::decode(parser, token_stream, &mut token_state)
        .map_err(|_| Error::Parse)?;
    println!("{:#?}", exprs);
    Ok(exprs.unwrap_or(Vec::new()))
}