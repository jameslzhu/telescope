use std::fmt;
use std::rc::Rc;
use token::Literal;

#[derive(Clone, Debug)]
pub enum Expr {
    Func(Function),
    Atom(Atom),
    List(List),
    Quote(Quote),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Atom {
    Bool(bool),
    Int(i64),
    Flt(f64),
    Str(String),
    Symbol(Symbol),
}

#[derive(Clone, Debug)]
pub struct List(Vec<Expr>);

#[derive(Clone, Debug)]
pub struct Quote(Vec<Expr>);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Symbol(String);

#[derive(Clone)]
pub struct Function {
    name: String,
    func: Rc<Fn(&[Expr]) -> Expr>,
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#[{}]", self.name)
    }
}

impl From<Literal> for Atom {
    fn from(x: Literal) -> Self {
        match x {
            Literal::Bool(y) => Atom::Bool(y),
            Literal::Int(y) => Atom::Int(y),
            Literal::Flt(y) => Atom::Flt(y),
            Literal::Str(y) => Atom::Str(y),
        }
    }
}
