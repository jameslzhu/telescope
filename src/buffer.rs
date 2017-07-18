use std::io;
use std::io::prelude::*;
use rustyline;
use rustyline::error::ReadlineError as RLError;
use combine;
use error::*;

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

#[derive(Clone, Debug)]
pub struct StringStream<R: io::Read> {
    src: R,
    buffer: String,
}

impl<R: io::Read> StringStream<R> {
    fn new(src: R) -> Self {
        StringStream {
            src: src,
            buffer: String::with_capacity(128),
        }
    }
}

impl<R: io::Read> combine::StreamOnce for StringStream<R> {
    type Item = char;
    type Range = str;
    type Position usize;
    fn uncons(&mut self) -> Result<Self::Item, Error<Self::Item, Self::Range>>;
    fn position(&self) -> Self::Position;
}
