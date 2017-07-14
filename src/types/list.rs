use super::Expr;
use itertools::Itertools;
use std::fmt;

#[derive(Clone, Debug)]
pub struct List(pub Vec<Expr>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.0.iter().join(" "))
    }
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        self.0.len() == other.0.len() && self.0.iter().zip(&other.0).all(|(a, b)| a == b)
    }
}
