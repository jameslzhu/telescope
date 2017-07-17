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

    let sym = args[0].sym()
        .ok_or("#[def] expected symbol")?;

    args[1].eval(env)
        .map(|expr| env.define(&sym.0, expr))
        .map(|_| args[0].clone())
}

// (if cond then else?)
fn if_form(args: &[Expr], env: &mut Env) -> Result<Expr> {
    if args.len() == 2 {
        if args[0].eval(env)?.truthiness() {
            args[1].eval(env)
        } else {
            Ok(Expr::Nil)
        }
    } else if args.len() == 3 {
        if args[0].eval(env)?.truthiness() {
            args[1].eval(env)
        } else {
            args[2].eval(env)
        }
    } else {
        Err("#[if] expected body clause".into())
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
    let name = args[0].clone().atom().and_then(|a| a.sym().cloned()).map(|n| n.0);
    let raw_params = if name.is_some() { &args[1] } else { &args[0] };
    let params = raw_params
        .vector()
        .ok_or("#[fn] expected arg vector")?
        .0
        .iter()
        .map(|ref x| x.sym().cloned())
        .collect::<Option<Vec<_>>>()
        .ok_or("#[fn] expected argument symbol")?;
    let body = if name.is_some() { args[2..].to_vec() } else { args[1..].to_vec() };
    Ok(Expr::from(Function::User { name, params, body }))
}
