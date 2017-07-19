use super::expr::Expr;
use itertools::Itertools;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Vector(pub Vec<Expr>);

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.iter().join(" "))
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
    	self.0 == other.0
    }
}
