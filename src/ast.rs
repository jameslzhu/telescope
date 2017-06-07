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
    name: Option<String>,
    func: Rc<Fn(&[Expr]) -> Expr>,
}

impl Function {
    fn new<S, F>(name: Option<S>, func: F) -> Self
        where S: Into<String>,
              F: 'static + Fn(&[Expr]) -> Expr
    {
        Function { name: name.map(Into::into), func: Rc::new(func) }
    }

    fn call<'a>(&self, args: &'a [Expr]) -> Expr {
        (self.func)(args)
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref name) = self.name {
            write!(f, "#[{}]", name)
        } else {
            write!(f, "#\u{03BB}")
        }
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

impl From<Quote> for List {
    fn from(x: Quote) -> Self {
        List(x.0)
    }
}

impl From<List> for Quote {
    fn from(x: List) -> Self {
        Quote(x.0)
    }
}

impl From<Symbol> for Atom {
    fn from(x: Symbol) -> Self {
        Atom::Symbol(x)
    }
}

fn lift<F, G>(func: Box<Fn(&[Atom]) -> Atom>) -> Box<Fn(&[Expr]) -> Expr>
{
    Box::new(move |args| Expr::Atom(
        func(args.iter()
            .map(|arg| { if let &Expr::Atom(ref atom) = arg { atom.clone() } else { panic!() } })
            .collect::<Vec<_>>()
            .as_slice())
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    fn add_fn(x: &[Expr]) -> Expr {
        Expr::Atom(Atom::Int(
            x.iter().map(|x| match x {
                &Expr::Atom(Atom::Int(y)) => y,
                _ => panic!()
            }).sum()))
    }
    #[test]
    fn print_debug() {
        let func = move |atoms: &[Atom]| Atom::Int(atoms.iter().map(|x| if let &Atom::Int(y) = x {y} else { panic!() }).sum());
        let add = Function::new(Some("add"), lift(Box::new(func)));
        add.call(vec![Expr::Atom(Atom::Int(1)), Expr::Atom(Atom::Int(2))].as_slice());
    }
}
