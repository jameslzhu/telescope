use std::fmt;
use itertools::Itertools;
use super::expr::Expr;

#[derive(Clone, Debug)]
pub struct Vector(pub Vec<Expr>);

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.iter().join(" "))
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.0.len() == other.0.len() && self.0.iter().zip(&other.0).all(|(a, b)| a == b)
    }
}
