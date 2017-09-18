use std::collections::HashMap;
use itertools::Itertools;

use env::Env;
use error::*;
use types::{Expr, Function, Macro, Symbol, Lambda};
use util::*;

lazy_static! {
    static ref SPECIAL_FORMS: HashMap<&'static str, Lambda> = {
        let forms: Vec<(&'static str, Lambda)> = vec![
            ("def", def_form),
            ("if",  if_form),
            ("let", let_form),
            ("do",  do_form),
            ("fn",  fn_form),
            ("macro", macro_form),
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

pub fn eval(form: &Symbol, args: &[Expr], env: Env) -> Result<Expr> {
    debug_assert!(is_special_form(form));
    (SPECIAL_FORMS.get(form.0.as_str()))
        .ok_or_else(|| format!("{} form not found", form.0).into())
        .and_then(|f| (f)(args, env))
}

fn def_impl(args: &[Expr], env: Env) -> Result<Expr> {
    let sym = ensure_sym("def", &args[0])?;
    args[1].eval(env.clone())
        .map(|expr| env.define(&sym.0, expr))
        .map(Expr::from)
}

// (def symbol init)
fn def_form(args: &[Expr], env: Env) -> Result<Expr> {
    ensure_args("def", args, 2)?;
    def_impl(&args, env)
}

// (if cond then else?)
fn if_form(args: &[Expr], env: Env) -> Result<Expr> {
    ensure_range_args("if", args, 2, 3)?;
    if args[0].eval(env.clone())?.truthiness() {
        args[1].eval(env.clone())
    } else {
        args.get(2).unwrap_or(&Expr::Nil).eval(env.clone())
    }
}

// (do exprs*)
fn do_form(args: &[Expr], env: Env) -> Result<Expr> {
    Expr::eval_all(args, env.clone())
}

// (let [bindings*] exprs*)
fn let_form(args: &[Expr], env: Env) -> Result<Expr> {
    let let_env = Env::new(HashMap::new(), Some(env));
    let bindings = ensure_vector("let", &args[0])?;

    for i in (0..bindings.0.len()).step(2) {
        def_impl(&(bindings.0)[i..i+2], let_env.clone())?;
    }

    Expr::eval_all(&args[1..], let_env.clone())
}

// (quote form)
fn quote_form(args: &[Expr], _env: Env) -> Result<Expr> {
    ensure_args("quote", args, 1)?;
    Ok(args[0].clone())
}

// (fn name? [params* ] exprs*)
fn fn_form(args: &[Expr], env: Env) -> Result<Expr> {
    ensure_min_args("fn", args, 2)?;
    let name = args[0].sym().cloned().map(|n| n.0);
    let raw_params = if name.is_some() { &args[1] } else { &args[0] };
    let params = ensure_vector("fn", raw_params)?
        .0.iter()
        .map(|x| ensure_sym("fn", x).map(|x| x.clone()))
        .collect::<Result<Vec<_>>>()?;
    let body = if name.is_some() { args[2..].to_vec() } else { args[1..].to_vec() };
    Ok(Expr::from(Function::User { name, params, body, env: env.clone() }))
}

// (macro name? [params* ] exprs*)
fn macro_form(args: &[Expr], _env: Env) -> Result<Expr> {
    ensure_min_args("macro", args, 2)?;
    let name = args[0].sym().cloned().map(|n| n.0);
    let raw_params = if name.is_some() { &args[1] } else { &args[0] };
    let params = ensure_vector("macro", raw_params)?
        .0.iter()
        .map(|x| ensure_sym("macro", x).map(|x| x.clone()))
        .collect::<Result<Vec<_>>>()?;
    let body = if name.is_some() { args[2..].to_vec() } else { args[1..].to_vec() };
    Ok(Expr::from(Macro::new(name, params, body)))
}

// (and exprs*)
fn and_form(args: &[Expr], env: Env) -> Result<Expr> {
    if let Some((last, rest)) = args.split_last() {
        // Return on first untruthy value
        for arg in rest {
            let value = arg.eval(env.clone())?;
            if !value.truthiness() {
                return Ok(value);
            }
        }
        last.eval(env.clone())
    } else {
        // (and) returns #t
        Ok(Expr::from(true))
    }
}

// (or exprs*)
fn or_form(args: &[Expr], env: Env) -> Result<Expr> {
    if let Some((last, rest)) = args.split_last() {
        // Return on first truthy value
        for arg in rest {
            let value = arg.eval(env.clone())?;
            if value.truthiness() {
                return Ok(value);
            }
        }
        last.eval(env.clone())
    } else {
        //.clone() (or) returns #t
        Ok(Expr::from(false))
    }
}
