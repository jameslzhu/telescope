use std::collections::VecDeque;

use combine::{StreamOnce, Positioned};
use combine::stream::state::{Positioner, SourcePosition};
use combine::easy::Error;
// use combine::error;
// use combine::stream;

use token::Token;

// #[derive(Clone, Debug)]
// pub struct StringStream {
//     line: Vec<char>,
//     position: usize,
// }

// impl StringStream {
//     pub fn new(line: &str) -> Self {
//         StringStream {
//             line: line.chars().collect(),
//             position: 0,
//         }
//     }
// }

// impl<'a> StreamOnce for StringStream {
//     type Item = char;
//     type Range = char;
//     type Position = usize;

//     type Error = error::StringStreamError;

//     fn uncons(&mut self) -> Result<Self::Item, stream::StreamErrorFor<Self>> {
//         let ch = self.line.as_slice().uncons();
//         self.position += 1;
//         ch
//     }
// }

// impl<'a> Positioned for StringStream {
//     fn position(&self) -> Self::Position {
//         self.position
//     }
// }

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

}

impl Positioned for TokenStream {
    fn position(&self) -> Self::Position {
        self.position
    }
}