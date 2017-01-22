extern crate rustyline;
extern crate combine;
extern crate telescope;

use rustyline::Editor;
use rustyline::error::ReadlineError as RLError;
use telescope::parse;

fn main() {
    // Prompt constants
    let header = format!(r"telescope v{}", env!("CARGO_PKG_VERSION"));
    let prompt = "> ";

    let mut rl = Editor::<()>::new();

    println!("{}", header);

    loop {
        match rl.readline(prompt) {
            Ok(line) => {
                rl.add_history_entry(&line);
                if line == "exit" || line == "quit" {
                    break;
                }
                match parse(line)
                    .map_err(|e| e.into())
                    .and_then(|node| node.eval()) {
                    Ok(v) => println!("{}", v),
                    Err(e) => println!("Error: {}", e),
                }
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
