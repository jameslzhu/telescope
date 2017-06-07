extern crate rustyline;
extern crate combine;
extern crate telescope;

use telescope::{lexer, parser};

use rustyline::Editor;
use rustyline::error::ReadlineError as RLError;

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
                match lexer::lex(line.trim_right()) {
                    Ok((tokens, _)) => {
                        match parser::parse_tokens(tokens.as_slice()) {
                            Ok((exprs, _)) => {
                                for expr in exprs {
                                    print!("{} ", expr);
                                }
                                println!();
                            }
                            Err(e) => println!("Error: {:?}", e),
                        }
                    },
                    Err(e) => println!("Error: {:?}", e),
                };
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