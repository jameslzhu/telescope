
#[derive(Debug, Fail)]
#[fail(display = "Lexing error")]
pub struct LexError;

#[derive(Debug, Fail)]
#[fail(display = "Parsing error")]
pub struct ParseError;