use std::fs;
use std::io;
use std::io::prelude::*;

use {lexer, parser, types};
use types::Expr;
use error::*;
use eval::Env;
use token::Token;
use buffer::Readline;
use stream::{StringStream, TokenStream};

pub fn file(path: &str, mut env: &mut Env) -> Result<()> {
    let file = fs::File::open(path)?;
    let mut file_buf = io::BufReader::new(file);
    loop {
        let exprs = read(&mut file_buf)?;
        eval(exprs, &mut env)?;
    }
}

pub fn repl(mut env: &mut Env) -> Result<i32> {
    let mut rl = Readline::new("> ");
    loop {
        let exprs = match read(&mut rl) {
            Ok(x) => x,
            Err(err) => {
                println!("{}", err);
                continue;
            },
        };
        match eval(exprs, &mut env) {
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

        let (tokens, _) = lexer::lex(StringStream::new(&line))?;
        token_buf.extend(tokens);

        let (exprs, unparsed) = parser::parse(TokenStream::new(token_buf.drain(..)))?;

        token_buf = unparsed.unwrap();
        expr_buf.extend(exprs);

        if token_buf.is_empty() {
            break;
        }
    }
    Ok(expr_buf)
}

fn eval(exprs: Vec<Expr>, env: &mut Env) -> Result<Expr> {
    if let Some((last, rest)) = exprs.split_last() {
        for expr in rest {
            expr.eval(env)?;
        }
        last.eval(env)
    } else {
        Ok(types::Expr::Nil)
    }
}

fn print(value: Expr) {
    if value != types::Expr::Nil {
        println!("{}", value);
    }
}
