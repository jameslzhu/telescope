extern crate linenoise;
extern crate lisp_rs;

use lisp_rs::parse;

fn main() {
    let header = format!("lrs v{}", env!("CARGO_PKG_VERSION"));
    let prompt = "> ";

    println!("{}", header);

    while let Some(input) = linenoise::input(prompt) {
        linenoise::history_add(&input);

        match input.as_str() {
            "clear" => linenoise::clear_screen(),
            "exit" | "quit" => break,
            _ => {
                let parsed = parse::parse_Expr(&input);

                match parsed {
                    Ok(result) => {
                        println!("{}", result);
                        match result.eval() {
                            Ok(value) => println!("{}", value),
                            Err(e) => println!("{}", e),
                        }
                    },
                    Err(e) => println!("error: {:?}", e),
                }
            }
        };
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
