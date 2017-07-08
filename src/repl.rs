use combine;
use rustyline::Editor;
use rustyline::error::ReadlineError as RLError;
use {lexer, parser, ast, ops};
use eval::Env;
use error::*;
use token::Token;

pub fn repl() -> Result<()> {
    // Prompt constants
    let header = format!(r"telescope v{}", env!("CARGO_PKG_VERSION"));
    let prompt = "> ";

    let mut rl = Editor::<()>::new();

    println!("{}", header);

    let mut env = ops::env();
    let mut token_buf = Vec::<Token>::with_capacity(1024);

    loop {
        match rl.readline(prompt) {
            Ok(line) => {
                rl.add_history_entry(&line);
                exec(&line, &mut token_buf, &mut env)?;
            }
            Err(RLError::Interrupted) |
            Err(RLError::Eof) => bail!(ErrorKind::Eof),
            Err(err) => {
                println!("Error: {:?}", err);
                bail!(ErrorKind::Eof);
            }
        };
    }
}

fn exec(line: &str, token_buf: &mut Vec<Token>, mut env: &mut Env) -> Result<()> {
    let (mut tokens, _unlexed) = lexer::lex(line.trim_right()).unwrap();
    let mut all_tokens = token_buf.drain(..).collect::<Vec<_>>();
    all_tokens.append(&mut tokens);
    match parser::parse(combine::State::new(&*all_tokens)) {
        Ok((expr, unparsed)) => {
            token_buf.extend_from_slice(unparsed.input);
            match eval(&expr, &mut env) {
                Ok(result) => Ok(print(result)),
                Err(err) => match err.kind() {
                    &ErrorKind::Exit(_) => Err(err),
                    _ => Ok(println!("Error: {}", err)),
                }
            }
        },
        Err(err) => Ok(println!("Parsing error: {:?}", err)),
    }
}

fn read(line: &str) -> ::std::result::Result<(ast::Expr, &[Token]), combine::ParseError<&[Token]>> {
    unimplemented!()
}

fn eval(expr: &ast::Expr, mut env: &mut Env) -> Result<ast::Expr> {
    expr.eval(&mut env)
}

fn print(value: ast::Expr) {
    if value != ast::Expr::Nil {
        println!("{}", value);
    }
}
