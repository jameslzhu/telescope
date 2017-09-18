use std::fmt;
use itertools::Itertools;

use super::{Expr, Symbol};

#[derive(Debug)]
pub struct Macro {
    pub name: Option<String>,
    pub params: Vec<Symbol>,
    pub body: Vec<Expr>,
}

impl Macro {
    pub fn new<S>(name: S, params: Vec<Symbol>, body: Vec<Expr>) -> Self
    where
        S: Into<Option<String>>
    {
        Macro { name: name.into(), params, body }
    }
}

impl fmt::Display for Macro {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(macro [{}] {})",
            self.params.iter().join(" "),
            self.body.iter().join("\n")
        )
    }
}
