use std::fmt;
use itertools::Itertools;
use super::expr::Expr;
use super::symbol::Symbol;
use eval::Env;
use error::*;

pub enum Function {
    Builtin { name: String, func: Lambda },
    User {
        name: Option<String>,
        params: Vec<Symbol>,
        body: Vec<Expr>,
    },
}

pub type Lambda = fn(&[Expr], &Env) -> Result<Expr>;

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
    #[cfg_attr(rustfmt, rustfmt_skip)]
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Function::Builtin { ref name, ref func }
                => f.debug_struct("Function::Builtin")
                    .field("name", &name)
                    .finish(),
            &Function::User { ref name, ref params, ref body, }
                => f.debug_struct("Function::Builtin")
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
            &Function::Builtin { ref name, ref func }
                => write!(f, "#[{}]", name),
            &Function::User { ref name, ref params, ref body, }
                => write!(f, "(fn [{}] {})", params.iter().join(" "), body.iter().join("\n")),
        }
    }
}
