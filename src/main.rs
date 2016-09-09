extern crate rustyline;
extern crate lrs;


use lrs::parse;
use rustyline::Editor;
use rustyline::error::ReadlineError as RLError;

fn main() {
    // Prompt constants
    let header = format!("lrs v{}", env!("CARGO_PKG_VERSION"));
    let prompt = "> ";

    let mut rl = Editor::new();

    println!("{}", header);

    loop {
        let readline = rl.readline(prompt);
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                match line.as_str() {
                    "exit" | "quit" => break,
                    _ => {
                        let parsed = parse::parse_Node(&line);

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

#[test]
fn calc() {
    assert!(parse::parse_Expr("22").is_ok());
    assert!(parse::parse_Expr("(22)").is_ok());
    assert!(parse::parse_Expr("((((22))))").is_ok());
    assert!(parse::parse_Expr("((( ( 22 ) )))").is_ok());
    assert!(parse::parse_Expr("((22)").is_err());
}

#[test]
fn calc2() {
    assert!(parse::parse_Expr("(+ 1 2 3)").is_ok());
    assert!(parse::parse_Expr("(+ 4 (/ 5 6))").is_ok());
}
