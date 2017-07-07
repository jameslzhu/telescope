#![allow(unused_variables)]

use std::collections::HashMap;
use ast::{Atom, Expr, Env, Symbol};
use itertools::Itertools;
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

fn def_form(args: &[Expr], env: &mut Env) -> Result<Expr> {
    if args.len() != 2 {
        return Err("#[def] expected 2 arguments".into());
    }

    if let Expr::Atom(Atom::Sym(ref sym)) = args[0] {
        env.define(&sym.0, args[1].clone());
        Ok(Expr::Nil)
    } else {
        Err("#[def] expected symbol".into())
    }
}

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

fn let_form(args: &[Expr], env: &mut Env) -> Result<Expr> {
    unimplemented!()
}

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

fn quote_form(args: &[Expr], env: &mut Env) -> Result<Expr> {
    unimplemented!()
}

fn fn_form(args: &[Expr], env: &mut Env) -> Result<Expr> {
    unimplemented!()
}
