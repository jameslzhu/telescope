use std::fs;
use std::io;
use std::io::prelude::*;

use syntax;
use types::Expr;
use error::*;
use env::Env;
use buffer::Readline;
// use stream::{StringStream, TokenStream};

pub fn file(path: &str, env: Env) -> Result<()> {
    let file = fs::File::open(path).map_err(Error::Io)?;
    let mut unparsed = String::with_capacity(128);
    let mut file_buf = io::BufReader::new(file);
    loop {
        let exprs = read(&mut file_buf, &mut unparsed)?;
        eval(&*exprs, env.clone())?;
    }
}

pub fn repl(env: Env) -> Result<i32> {
    let mut rl = Readline::new("> ");
    let mut unparsed = String::new();
    loop {
        let exprs = match read(&mut rl, &mut unparsed) {
            Ok(x) => x,
            Err(err) => {
                println!("{}", err);
                continue;
            },
        };
        match eval(&exprs, env.clone()) {
            Ok(val) => print(&val),
            Err(err) => {
                match err {
                    Error::Eof => return Ok(0),
                    Error::Exit(code) => return Ok(code),
                    _ => println!("{}", err),
                }
            }
        };
    }
}

fn read<B: BufRead>(reader: &mut B, unparsed: &mut String) -> Result<Vec<Expr>> {
    let mut token_buffer = Vec::new();
    let mut token_state = syntax::empty_state();
    // let mut token_buf: Vec<Token> = Vec::with_capacity(128);
    let mut expr_buf: Vec<Expr> = Vec::with_capacity(16);
    let mut lines = reader.lines();

    while expr_buf.is_empty() {
        let mut line = match lines.next() {
            Some(Ok(l)) => l,
            Some(Err(err)) => return Err(Error::Io(err)),
            None => return Err(Error::Eof),
        };
        unparsed.extend(line.drain(..));

        let exprs = syntax::parse_line(&unparsed, &mut token_buffer, &mut token_state)?;
        expr_buf.extend(exprs)
    }
    Ok(expr_buf)
}

fn eval(exprs: &[Expr], env: Env) -> Result<Expr> {
    if let Some((last, rest)) = exprs.split_last() {
        for expr in rest {
            expr.eval(env.clone())?;
        }
        last.eval(env.clone())
    } else {
        Ok(Expr::Nil)
    }
}

fn print(value: &Expr) {
    if value != &Expr::Nil {
        println!("{}", value);
    }
}
