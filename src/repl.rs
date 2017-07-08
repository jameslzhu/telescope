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

fn exec(line: &str, mut token_buf: &mut Vec<Token>, mut env: &mut Env) -> Result<()> {
    match read(&line, &mut token_buf) {
        Ok(expr) => {
            eval(&expr, &mut env)
                .map(|value| print(Ok(value)))
                .or_else(|err| {
                    if let &ErrorKind::Exit(_) = err.kind() {
                        Err(err)
                    } else {
                        print(Err(err));
                        Ok(())
                    }
                })
        },
        Err(err) => Ok(println!("Parsing error: {}", err)),
    }
}

fn read(line: &str, mut token_buf: &mut Vec<Token>)
    -> Result<(ast::Expr)>
{
    let (tokens, _unlexed) = lexer::lex(line.trim_right()).unwrap();
    let all_tokens = token_buf.drain(..).chain(tokens).collect::<Vec<_>>();
    let token_iter = combine::from_iter(all_tokens.into_iter());
    parser::parse(combine::State::new(token_iter))
        .map(|(expr, unparsed)| {
            token_buf.extend(unparsed.input);
            expr
        })
        .map_err(|x| x.into())
}

fn eval(expr: &ast::Expr, mut env: &mut Env) -> Result<ast::Expr> {
    expr.eval(&mut env)
}

fn print(result: Result<ast::Expr>) {
    match result {
        Ok(value) => {
            if value != ast::Expr::Nil {
                println!("{}", value);
            }
        }
        Err(err) => {
            println!("Error: {}", err)
        }
    }
}
