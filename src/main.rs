extern crate rustyline;
extern crate combine;
extern crate telescope;

use rustyline::Editor;
use rustyline::error::ReadlineError as RLError;
use combine::parser;
use combine::{State, Parser};
use telescope::parser::lang;

fn main() {
    // Prompt constants
    let header = format!(r"telescope v{}", env!("CARGO_PKG_VERSION"));
    let prompt = "> ";

    let mut rl = Editor::<()>::new();

    println!("{}", header);

    loop {
        let readline = rl.readline(prompt);
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                if line == "exit" || line == "quit" {
                    break;
                }
                let _ = parser(lang).parse(State::new(line.as_str()))
                    .map_err(|e| e.into())
                    .and_then(|(node, _)| node.eval())
                    .map(|v| println!("{}", v))
                    .map_err(|e| println!("Error: {}", e));
            }
            Err(RLError::Interrupted) |
            Err(RLError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        };
    }
}
