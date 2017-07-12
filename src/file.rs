use {lexer, parser, ast, ops};
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use combine;
use error::*;
use eval::Env;
use token::Token;

pub fn run(path: &str) -> Result<()> {
    let mut env = ops::env();
    let mut token_buf = Vec::<Token>::with_capacity(1024);

    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut source = String::with_capacity(128);
    buf_reader.read_to_string(&mut source)?;
    exec(combine::from_iter(source.chars()), &mut token_buf, &mut env)
}

fn exec<S>(input: S, mut token_buf: &mut Vec<Token>, mut env: &mut Env) -> Result<()>
where
    S: combine::Stream<Item = char>
{
    match read(input, &mut token_buf) {
        Ok(exprs) => {
            eval(&exprs, &mut env)
                .map(|value| print(Ok(value)))
                .or_else(|err| if let &ErrorKind::Exit(_) = err.kind() {
                    Err(err)
                } else {
                    print(Err(err));
                    Ok(())
                })
        }
        Err(err) => Ok(println!("Parsing error: {}", err)),
    }
}

fn read<S>(input: S, mut token_buf: &mut Vec<Token>) -> Result<Vec<ast::Expr>>
where
    S: combine::Stream<Item = char>
{
    let tokens = lexer::lex(input)
        .map(|x| x.0)
        .unwrap_or(Vec::new());
    let all_tokens = token_buf.drain(..).chain(tokens).collect::<Vec<_>>();
    let token_iter = combine::from_iter(all_tokens.into_iter());
    let (exprs, unparsed) = parser::parse(combine::State::new(token_iter))?;
    token_buf.extend(unparsed.input);
    Ok(exprs)
}

fn eval(exprs: &[ast::Expr], mut env: &mut Env) -> Result<ast::Expr> {
    match exprs.split_last() {
        Some((last, rest)) => {
            for expr in rest {
                expr.eval(&mut env)?;
            }
            last.eval(&mut env)
        }
        None => Ok(ast::Expr::Nil),
    }
}

fn print(result: Result<ast::Expr>) {
    match result {
        Ok(value) => {
            if value != ast::Expr::Nil {
                println!("{}", value);
            }
        }
        Err(err) => println!("Error: {}", err),
    }
}
