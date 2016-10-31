extern crate rustyline;
extern crate telescope;

use telescope::parse;
use rustyline::Editor;
use rustyline::error::ReadlineError as RLError;

fn main() {
    // Prompt constants
    let header = format!("telescope v{}", env!("CARGO_PKG_VERSION"));
    let prompt = "> ";

    let mut rl = Editor::<()>::new();

    println!("{}", header);

    loop {
        let readline = rl.readline(prompt);
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                match line.as_str() {
                    "exit" | "quit" => break,
                    _ => {
                        let parsed = parse::parse_Lang(&line);

                        match parsed {
                            Ok(result) => {
                                match result.eval() {
                                    Ok(value) => println!("{}", value),
                                    Err(e) => println!("{}", e),
                                }
                            }
                            Err(e) => println!("Error: {:?}", e),
                        }
                    }
                };
            }
            Err(RLError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(RLError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
