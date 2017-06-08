extern crate rustyline;
extern crate combine;
extern crate telescope;


use rustyline::Editor;
use rustyline::error::ReadlineError as RLError;
use telescope::{lexer, parser};

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
                    Ok((tokens, unlexed)) => {
                        match parser::parse(tokens.as_slice()) {
                            Ok((exprs, unparsed)) => {
                                for expr in exprs {
                                    print!("{} ", expr);
                                }
                                println!();
                                if !unparsed.is_empty() {
                                    println!("Unparsed: {:?}", unparsed);
                                }
                            }
                            Err(e) => println!("Error: {:?}", e),
                        };
                        if !unlexed.is_empty() {
                            println!("Unlexed: {}", unlexed)
                        }
                    }
                    Err(e) => println!("Error: {}", e),
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
