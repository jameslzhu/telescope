use super::*;
use std::sync::Arc;
use token::Literal;

impl<T> From<T> for Expr
where
    T: Into<Literal>,
{
    fn from(x: T) -> Self {
        match x.into() {
            Literal::Bool(y) => Expr::Bool(y),
            Literal::Int(y) => Expr::Int(y),
            Literal::Flt(y) => Expr::Flt(y),
            Literal::Str(y) => Expr::Str(y),
        }
    }
}

impl From<Symbol> for Expr {
    fn from(x: Symbol) -> Self {
        Expr::Sym(x)
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

impl From<Function> for Expr {
    fn from(x: Function) -> Expr {
        Expr::Func(Arc::new(x))
    }
}

impl From<Macro> for Expr {
    fn from(x: Macro) -> Expr {
        Expr::Macro(Arc::new(x))
    }
}
