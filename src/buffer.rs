use std::io::prelude::*;
use std::io;
use rustyline::Editor;
use rustyline::error::ReadlineError as RLError;
use combine::{StreamOnce};
use error::*;

#[derive(Debug)]
pub struct Buffer {
    buffer: String,
    prompt: String,
    rl: Editor<()>,
}

impl Buffer {
    fn new<S>(prompt: S) -> Self
    where
        S: Into<String>
    {
        Buffer {
            buffer: String::with_capacity(128),
            prompt: prompt.into(),
            rl: Editor::new(),
        }
    }
}

impl Read for Buffer {
    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        if self.buffer.is_empty() {
            match self.rl.readline(&self.prompt) {
                Ok(line) => {
                    self.rl.add_history_entry(&line);
                    self.buffer.push_str(&line);
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
            let result = buf.write(self.buffer.as_bytes());
            self.buffer.clear();
            result
        } else {
            let bytes = self.buffer.drain(0..buf.len()).collect::<String>();
            buf.write(bytes.into_bytes().as_slice())
        }
    }
}

impl BufRead for Buffer {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {

    }

    fn consume(&mut self, amt: usize) {

    }
}