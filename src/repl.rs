use rustyline::Editor;
use rustyline::error::ReadlineError as RLError;
use combine::ParseError;
use {lexer, parser, ast, ops};
use error::*;
use token::Token;

pub fn repl() {
    // Prompt constants
    let header = format!(r"telescope v{}", env!("CARGO_PKG_VERSION"));
    let prompt = "> ";

    let mut rl = Editor::<()>::new();

    println!("{}", header);

    let mut env = ops::env();

    loop {
        match rl.readline(prompt) {
            Ok(line) => {
                rl.add_history_entry(&line);
                if line == "exit" || line == "quit" {
                    break;
                }
                exec(&line, &mut env);
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

fn exec(line: &str, mut env: &mut ast::Env) {
    let (tokens, _unlexed) = lexer::lex(line.trim_right()).unwrap();
    match parser::parse(&*tokens) {
        Ok((expr, _unparsed)) => print(eval(&expr, &mut env)),
        Err(err) => println!("Parsing error: {:?}", err),
    };
}

fn read(line: &str) -> ::std::result::Result<(ast::Expr, &[Token]), ParseError<&[Token]>> {
    unimplemented!()
}

fn eval(expr: &ast::Expr, mut env: &mut ast::Env) -> Result<ast::Expr> {
    expr.eval(&mut env)
}

fn print(result: Result<ast::Expr>) {
    match result {
        Ok(value) => {
            if value != ast::Expr::Nil {
                println!("{}", value);
            }
        },
        Err(err) => println!("Error: {}", err),
    }
}
