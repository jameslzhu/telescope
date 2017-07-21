use std::fs;
use std::io;
use std::io::prelude::*;

use {lexer, parser, types};
use types::Expr;
use combine;
use combine::State;
use combine::primitives::IteratorStream;
use error::*;
use env::Env;
use token::Token;
use buffer::Readline;

pub fn file(path: &str, env: Env) -> Result<()> {
    let file = fs::File::open(path)?;
    let mut file_buf = io::BufReader::new(file);
    loop {
        let exprs = read(&mut file_buf)?;
        eval(exprs, env.clone())?;
    }
}

pub fn repl(env: Env) -> Result<i32> {
    let mut rl = Readline::new("> ");
    loop {
        let exprs = match read(&mut rl) {
            Ok(x) => x,
            Err(err) => {
                println!("{}", err);
                continue;
            },
        };
        match eval(exprs, env.clone()) {
            Ok(val) => print(val),
            Err(err) => {
                match err.kind() {
                    &ErrorKind::Eof => return Ok(0),
                    &ErrorKind::Exit(code) => return Ok(code),
                    _ => println!("{}", err),
                }
            }
        };
    }
}

fn read<B: BufRead>(reader: &mut B) -> Result<Vec<Expr>> {
    let mut token_buf: Vec<Token> = Vec::with_capacity(128);
    let mut expr_buf: Vec<Expr> = Vec::with_capacity(16);
    let mut lines = reader.lines();
    loop {
        let line = match lines.next() {
            Some(Ok(l)) => l,
            Some(Err(err)) => return Err(err.into()),
            None => return Err(ErrorKind::Eof.into()),
        };

        let (tokens, _) = match lexer::lex(State::new(&*line)) {
            Ok(x) => x,
            Err(err) => {
                println!("{}", err);
                break;
            },
        };

        token_buf.extend(tokens);
        let all_tokens = token_buf.drain(..).collect::<Vec<_>>();
        let token_iter = IteratorStream::new(all_tokens.into_iter());
        let (exprs, unparsed) = parser::parse(combine::State::new(token_iter))?;
        expr_buf.extend(exprs);
        token_buf.extend(unparsed.input);

        if token_buf.is_empty() {
            break;
        } else {
            println!("{:?}", token_buf);
        }
    }
    Ok(expr_buf)
}

fn eval(exprs: Vec<Expr>, env: Env) -> Result<Expr> {
    if let Some((last, rest)) = exprs.split_last() {
        for expr in rest {
            expr.eval(env.clone())?;
        }
        last.eval(env.clone())
    } else {
        Ok(types::Expr::Nil)
    }
}

fn print(value: Expr) {
    if value != types::Expr::Nil {
        println!("{}", value);
    }
}
