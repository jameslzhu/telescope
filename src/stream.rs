use std::collections::VecDeque;

use combine::StreamOnce;
use combine::primitives::{Error, Positioner, SourcePosition};

use token::Token;

#[derive(Clone, Debug)]
pub struct StringStream {
    line: Vec<char>,
    position: usize,
}

impl StringStream {
    pub fn new(line: &str) -> Self {
        StringStream {
            line: line.chars().collect(),
            position: 0,
        }
    }
}

impl<'a> StreamOnce for StringStream {
    type Item = char;
    type Range = char;
    type Position = usize;

    fn uncons(&mut self) -> Result<char, Error<char, char>> {
        let ch = self.line.get(self.position)
            .map(|x| *x)
            .ok_or(Error::end_of_input());
        self.position += 1;
        ch
    }

    fn position(&self) -> Self::Position {
        self.position
    }
}

#[derive(Clone, Debug)]
pub struct TokenStream {
    line: VecDeque<Token>,
    position: SourcePosition,
}

impl TokenStream {
    pub fn new<T>(line: T) -> Self
    where T: Iterator<Item = Token>
    {
        TokenStream {
            line: line.collect(),
            position: SourcePosition {column: 0, line: 0},
        }
    }

    pub fn unwrap(self) -> Vec<Token> {
        self.line.into()
    }
}

impl StreamOnce for TokenStream {
    type Item = Token;
    type Range = Token;
    type Position = SourcePosition;

    fn uncons(&mut self) -> Result<Token, Error<Token, Token>> {
        if let Some(token) = self.line.pop_front() {
            token.update(&mut self.position);
            Ok(token.clone())
        } else {
            Err(Error::end_of_input())
        }
    }

    fn position(&self) -> Self::Position {
        self.position
    }
}