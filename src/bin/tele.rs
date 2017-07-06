extern crate rustyline;
extern crate combine;
extern crate telescope;

use rustyline::Editor;
use rustyline::error::ReadlineError as RLError;
use telescope::{lexer, parser, ast};
// use telescope::error::*;

fn main() {
    // Prompt constants
    let header = format!(r"telescope v{}", env!("CARGO_PKG_VERSION"));
    let prompt = "> ";

    let mut rl = Editor::<()>::new();

    println!("{}", header);

    let env = telescope::env();

    loop {
        match rl.readline(prompt) {
            Ok(line) => {
                rl.add_history_entry(&line);
                if line == "exit" || line == "quit" {
                    break;
                }

                let _ = lexer::lex(line.trim_right())
                    .map(|(tokens, _unlexed)| {
                        // println!("{:?}", tokens);
                        parser::parse(&*tokens)
                            .map(|(exprs, _unparsed)| for expr in exprs {
                                match expr.eval(&env) {
                                    Ok(value) => {
                                        if value != ast::Expr::Nil {
                                            println!("{} ", value);
                                        }
                                    }
                                    Err(err) => {
                                        println!("Error: {}", err);
                                        continue;
                                    }
                                }
                            })
                            .map_err(|err| println!("Unparsed: {:?}", err))
                    })
                    .map_err(|err| println!("Error: {}", err));
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
