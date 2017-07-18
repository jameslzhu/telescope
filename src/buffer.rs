use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;
use rustyline;
use rustyline::error::ReadlineError as RLError;
use combine;
use combine::primitives::SourcePosition;

pub struct Readline {
    rl: rustyline::Editor<()>,
    prompt: String,
    buffer: Vec<u8>,
}

impl Readline {
    fn new<S>(prompt: &str) -> Self {
        Readline {
            rl: rustyline::Editor::new(),
            prompt: prompt.to_owned(),
            buffer: Vec::with_capacity(128),
        }
    }
}

impl Read for Readline {
    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        if self.buffer.is_empty() {
            match self.rl.readline(&self.prompt) {
                Ok(line) => {
                    self.rl.add_history_entry(&line);
                    self.buffer.write(line.as_bytes());
                },
                Err(RLError::Io(err)) => return Err(err),
                Err(RLError::Interrupted) => return Err(io::ErrorKind::Interrupted.into()),
                Err(RLError::Eof) => return Ok(0),
                #[cfg(unix)]
                Err(RLError::Errno(num)) => return Err(io::Error::from_raw_os_error(num)),
                Err(err) => return Err(io::ErrorKind::Other.into()),
            }
        }

        // if buf > buffer, drain entire buffer
        if buf.len() >= self.buffer.len() {
            let result = buf.write(&self.buffer);
            self.buffer.clear();
            result
        } else {
            let drained = self.buffer.drain(0..buf.len()).collect::<Vec<_>>();
            buf.write(&drained)
        }
    }
}

impl io::BufRead for Readline {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        unimplemented!()
    }

    fn consume(&mut self, amt: usize) {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct LineStream<B: io::BufRead> {
    lines: io::Lines<B>,
    buffer: VecDeque<char>,
    position: SourcePosition,
}

impl<B: io::BufRead> LineStream<B> {
    fn new(src: B) -> Self {
        LineStream {
            lines: src.lines(),
            buffer: VecDeque::with_capacity(128),
            position: SourcePosition { line: 0, column: 0 },
        }
    }
}

impl<B: io::BufRead> combine::StreamOnce for LineStream<B> {
    type Item = char;
    type Range = String;
    type Position = combine::primitives::SourcePosition;

    fn uncons(&mut self) -> Result<Self::Item, combine::primitives::Error<Self::Item, Self::Range>> {
        if self.buffer.is_empty() {
            match self.lines.next() {
                Some(result) => {
                    match result {
                        Ok(line) => self.buffer.extend(line.chars()),
                        Err(err) => return Err(combine::primitives::Error::Other(Box::new(err)))
                    };
                },
                None => return Err(combine::primitives::Error::end_of_input()),
            }
            self.position.line += 1;
            self.position.column = 0;
        }
        let item = self.buffer.pop_front().unwrap();
        self.position.column += 1;
        Ok(item)
    }

    fn position(&self) -> Self::Position {
        self.position
    }
}
