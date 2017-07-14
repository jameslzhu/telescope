use super::*;
use std::sync::Arc;
use token::Literal;

impl<T> From<T> for Atom
where
    T: Into<Literal>,
{
    fn from(x: T) -> Self {
        match x.into() {
            Literal::Bool(y) => Atom::Bool(y),
            Literal::Int(y) => Atom::Int(y),
            Literal::Flt(y) => Atom::Flt(y),
            Literal::Str(y) => Atom::Str(y),
        }
    }
}

impl From<Symbol> for Atom {
    fn from(x: Symbol) -> Self {
        Atom::Sym(x)
    }
}

impl From<Vector> for List {
    fn from(x: Vector) -> Self {
        List(x.0)
    }
}

impl From<List> for Vector {
    fn from(x: List) -> Self {
        Vector(x.0)
    }
}

impl<T> From<T> for Expr
where
    T: Into<Atom>,
{
    fn from(x: T) -> Expr {
        Expr::Atom(x.into())
    }
}

impl From<Function> for Expr {
    fn from(x: Function) -> Expr {
        Expr::Func(Arc::new(x))
    }
}
