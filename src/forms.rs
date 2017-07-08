#![allow(unused_variables)]

use std::collections::HashMap;
use ast::{Atom, Expr, Function, Symbol};
use eval::Env;
use error::*;

type Form = Fn(&[Expr], &mut Env) -> Result<Expr> + Sync;

lazy_static! {
    static ref SPECIAL_FORMS: HashMap<String, Box<Form>> = {
        let mut forms = HashMap::<String, Box<Form>>::new();
        forms.insert("def".into(), Box::new(def_form));
        forms.insert("if".into(), Box::new(if_form));
        forms.insert("let".into(), Box::new(let_form));
        forms.insert("do".into(), Box::new(do_form));
        forms.insert("fn".into(), Box::new(fn_form));
        forms.insert("quote".into(), Box::new(quote_form));
        forms
    };
}

pub fn is_special_form(form: &Symbol) -> bool {
    SPECIAL_FORMS.contains_key(&form.0)
}

pub fn eval(form: &Symbol, args: &[Expr], env: &mut Env) -> Result<Expr> {
    assert!(is_special_form(form));
    (SPECIAL_FORMS.get(&form.0))
        .ok_or("did not find special form".into())
        .and_then(|f| (f)(args, env))
}

// (def symbol init)
fn def_form(args: &[Expr], env: &mut Env) -> Result<Expr> {
    if args.len() != 2 {
        return Err("#[def] expected 2 arguments".into());
    }

    if let Expr::Atom(Atom::Sym(ref sym)) = args[0] {
        args[1].eval(env)
            .map(|expr| env.define(&sym.0, expr))
            .map(|_| args[0].clone())
    } else {
        Err("#[def] expected symbol".into())
    }
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
    if args.len() == 1 {
        Ok(args[0].clone())
    } else {
        Err("#[quote] expected 1 argument".into())
    }
}

// (fn name? [params* ] exprs*)
fn fn_form(args: &[Expr], env: &mut Env) -> Result<Expr> {
    if args.len() >= 2 {
        let name = args[0].clone().atom().and_then(|a| a.sym().cloned());
        if name.is_some() {
            let params = args[1].clone().vector()
                .ok_or("#[fn] expected arg vector")?
                .0
                .iter()
                .map(|ref x| x.atom().and_then(|x2| x2.sym().cloned()))
                .collect::<Option<Vec<_>>>()
                .ok_or("#[fn] expected argument symbol")?;
            let body = args[2..].to_vec();
            Ok(Expr::Func(Function::User {
                name: name.map(|n| n.0),
                params: params,
                body: body
            }))
        } else {
            let params = args[0].clone().vector()
                .ok_or("#[fn] expected arg vector")?
                .0
                .iter()
                .map(|ref x| x.atom().and_then(|x2| x2.sym().cloned()))
                .collect::<Option<Vec<_>>>()
                .ok_or("#[fn] expected argument symbol")?;
            let body = args[1..].to_vec();
            Ok(Expr::Func(Function::User {
                name: None,
                params: params,
                body: body
            }))
        }
    } else {
        Err("#[fn] expected arguments (fn name? [params] exprs*)".into())
    }
}
