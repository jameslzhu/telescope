#![cfg_attr(rustfmt, rustfmt_skip)]

use itertools::Itertools;

use super::expr::Expr;
use super::symbol::Symbol;
use env::Env;
use error::*;
use std::fmt;

pub enum Function {
    Builtin {
        name: String,
        func: Lambda
    },
    User {
        name: Option<String>,
        params: Vec<Symbol>,
        body: Vec<Expr>,
        env: Env,
    },
}

pub type Lambda = fn(&[Expr], Env) -> Result<Expr>;

impl Function {
    pub fn builtin<S>(name: S, func: Lambda) -> Self
    where
        S: Into<String>,
    {
        Function::Builtin {
            name: name.into(),
            func: func,
        }
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Function::Builtin { ref name, func: _ }
                => f.debug_struct("Function::Builtin")
                    .field("name", &name)
                    .finish(),
            &Function::User { ref name, ref params, ref body, env: _ }
                => f.debug_struct("Function::User")
                    .field("name", &name)
                    .field("params", &params)
                    .field("body", &body)
                    .finish(),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Function::Builtin { ref name, func: _ } => write!(f, "#[{}]", name),
            &Function::User { name: _, ref params, ref body, env: _ } => {
                write!( f, "(fn [{}] {})",
                    params.iter().join(" "),
                    body.iter().join("\n")
                )
            }
        }
    }
}
