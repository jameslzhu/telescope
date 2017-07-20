use std::io;
use std::io::prelude::*;
use std::mem;
use rustyline;
use rustyline::error::ReadlineError as RLError;

#[derive(Debug)]
pub struct Readline {
    rl: rustyline::Editor<()>,
    prompt: String,
    buffer: String,
}

impl Readline {
    pub fn new(prompt: &str) -> Self {
        Readline {
            rl: rustyline::Editor::new(),
            prompt: prompt.to_owned(),
            buffer: String::with_capacity(128),
        }
    }
}

impl Read for Readline {
    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        if self.buffer.is_empty() {
            match self.rl.readline(&self.prompt) {
                Ok(line) => {
                    self.rl.add_history_entry(&line);
                    self.buffer.push_str(&line);
                    self.buffer.push('\n');
                },
                Err(RLError::Io(err)) => return Err(err),
                Err(RLError::Eof) => return Ok(0),
                Err(RLError::Interrupted) => return Ok(0),
                #[cfg(unix)]
                Err(RLError::Char(_)) => return Ok(0),
                #[cfg(unix)]
                Err(RLError::Errno(err)) => return Err(io::Error::from(err)),
                #[cfg(windows)]
                Err(RLError::WindowResize) => return Err(io::ErrorKind::Other.into()),
                #[cfg(windows)]
                Err(RLError::Decode(_)) => return Err(io::ErrorKind::InvalidData.into()),
            };
        }

        // if buf > buffer, drain entire buffer
        if buf.len() >= self.buffer.len() {
            let result = buf.write(&self.buffer.as_bytes());
            self.buffer.clear();
            result
        } else {
            let mut copy = String::with_capacity(128);
            mem::swap(&mut self.buffer, &mut copy);
            buf.write(&copy.into_bytes())
        }
    }
}

impl io::BufRead for Readline {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        if self.buffer.is_empty() {
            match self.rl.readline(&self.prompt) {
                Ok(line) => {
                    self.rl.add_history_entry(&line);
                    self.buffer.push_str(&line);
                    self.buffer.push('\n');
                },
                Err(RLError::Io(err)) => return Err(err),
                Err(RLError::Eof) => (),
                Err(RLError::Interrupted) => (),
                #[cfg(unix)]
                Err(RLError::Char(_)) => (),
                #[cfg(unix)]
                Err(RLError::Errno(err)) => return Err(io::Error::from(err)),
                #[cfg(windows)]
                Err(RLError::WindowResize) => return Err(io::ErrorKind::Other.into()),
                #[cfg(windows)]
                Err(RLError::Decode(_)) => return Err(io::ErrorKind::InvalidData.into()),
            }
        }

        Ok(&self.buffer.as_bytes())
    }

    fn consume(&mut self, amt: usize) {
        self.buffer.drain(0..amt);
    }
}
