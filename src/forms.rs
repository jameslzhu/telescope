#![allow(unused_variables)]

use std::collections::HashMap;
use eval::Env;
use error::*;
use types::{Expr, Function, Symbol};
use util::*;

type Form = fn(&[Expr], &mut Env) -> Result<Expr>;

lazy_static! {
    static ref SPECIAL_FORMS: HashMap<&'static str, Form> = {
        let forms: Vec<(&'static str, Form)> = vec![
            ("def", def_form),
            ("if",  if_form),
            ("let", let_form),
            ("do",  do_form),
            ("fn",  fn_form),
            ("quote", quote_form),
            ("and", and_form),
            ("or", or_form),
        ];
        forms.into_iter().collect()
    };
}

pub fn is_special_form(form: &Symbol) -> bool {
    SPECIAL_FORMS.contains_key(form.0.as_str())
}

pub fn eval(form: &Symbol, args: &[Expr], env: &mut Env) -> Result<Expr> {
    debug_assert!(is_special_form(form));
    (SPECIAL_FORMS.get(form.0.as_str()))
        .ok_or("did not find special form".into())
        .and_then(|f| (f)(args, env))
}

// (def symbol init)
#[cfg_attr(rustfmt, rustfmt_skip)]
fn def_form(args: &[Expr], env: &mut Env) -> Result<Expr> {
    ensure_args("def", args, 2)?;

    let sym = ensure_sym("def", &args[0])?;

    args[1].eval(env)
        .map(|expr| env.define(&sym.0, expr))
        .map(|_| args[0].clone())
}

// (if cond then else?)
fn if_form(args: &[Expr], env: &mut Env) -> Result<Expr> {
    ensure_range_args("if", args, 2, 3)?;
    if args[0].eval(env)?.truthiness() {
        args[1].eval(env)
    } else {
        args.get(2).unwrap_or(&Expr::Nil).eval(env)
    }
}

// (let [bindings*] exprs*)
fn let_form(args: &[Expr], env: &mut Env) -> Result<Expr> {
    unimplemented!()
}

// (do exprs*)
fn do_form(args: &[Expr], env: &mut Env) -> Result<Expr> {
    match args.split_last() {
        Some((last, rest)) => {
            for arg in rest {
                arg.eval(env)?;
            }
            last.eval(env)
        }
        None => Ok(Expr::Nil),
    }
}

// (quote form)
fn quote_form(args: &[Expr], env: &mut Env) -> Result<Expr> {
    ensure_args("quote", args, 1)?;
    Ok(args[0].clone())
}

// (fn name? [params* ] exprs*)
fn fn_form(args: &[Expr], env: &mut Env) -> Result<Expr> {
    ensure_min_args("fn", args, 2)?;
    let name = args[0].sym().cloned().map(|n| n.0);
    let raw_params = if name.is_some() { &args[1] } else { &args[0] };
    let params = ensure_vector("fn", raw_params)?
        .0.iter()
        .map(|x| ensure_sym("fn", x).map(|x| x.clone()))
        .collect::<Result<Vec<_>>>()?;
    let body = if name.is_some() { args[2..].to_vec() } else { args[1..].to_vec() };
    Ok(Expr::from(Function::User { name, params, body }))
}

// (and exprs*)
fn and_form(args: &[Expr], env: &mut Env) -> Result<Expr> {
    if let Some((last, rest)) = args.split_last() {
        // Return on first untruthy value
        for arg in rest {
            let value = arg.eval(env)?;
            if !value.truthiness() {
                return Ok(value);
            }
        }
        last.eval(env)
    } else {
        // (and) returns #t
        Ok(Expr::from(true))
    }
}

// (or exprs*)
fn or_form(args: &[Expr], env: &mut Env) -> Result<Expr> {
    if let Some((last, rest)) = args.split_last() {
        // Return on first truthy value
        for arg in rest {
            let value = arg.eval(env)?;
            if value.truthiness() {
                return Ok(value);
            }
        }
        last.eval(env)
    } else {
        // (or) returns #t
        Ok(Expr::from(false))
    }
}
