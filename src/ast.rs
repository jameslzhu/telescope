use std::fmt;
use std::rc::Rc;
use token::Literal;
use error::*;

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
    Sym(Symbol),
}

#[derive(Clone, Debug)]
pub struct List(pub Vec<Expr>);

#[derive(Clone, Debug)]
pub struct Quote(pub Vec<Expr>);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Symbol(pub String);

#[derive(Clone)]
pub struct Function {
    name: Option<String>,
    func: Rc<Lambda>,
}

pub type Lambda = Box<Fn(&[Expr]) -> Result<Expr>>;

impl Function {
    pub fn new<S>(name: Option<S>, func: Lambda) -> Self
        where S: Into<String>,
    {
        Function { name: name.map(Into::into), func: Rc::new(func) }
    }

    pub fn call<'a>(&self, args: &'a [Expr]) -> Result<Expr> {
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

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expr::Func(ref func) => write!(f, "{}", func),
            Expr::Atom(ref atom) => write!(f, "{}", atom),
            Expr::List(ref list) => write!(f, "{}", list),
            Expr::Quote(ref quote) => write!(f, "{}", quote),
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Atom::Bool(boolean) => write!(f, "#{}", if boolean { "t" } else { "f" }),
            Atom::Int(int) => write!(f, "{}", int),
            Atom::Flt(flt) => write!(f, "{}", flt),
            Atom::Str(ref string) => write!(f, "\"{}\"", string),
            Atom::Sym(ref sym) => write!(f, "#{}", sym.0),
        }
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(");
        let mut iter = self.0.iter();
        iter.next().map(|x| write!(f, "{}", x));
        for expr in iter {
            write!(f, " {}", expr);
        }
        write!(f, ")")
    }
}

impl fmt::Display for Quote {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[");
        let mut iter = self.0.iter();
        iter.next().map(|x| write!(f, "{}", x));
        for expr in iter {
            write!(f, " {}", expr);
        }
        write!(f, "]")
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl<T> From<T> for Expr where T: Into<Atom>
{
    fn from(x: T) -> Expr {
        Expr::Atom(x.into())
    }
}

impl From<Function> for Expr {
    fn from(x: Function) -> Expr {
        Expr::Func(x)
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

impl From<Symbol> for Atom {
    fn from(x: Symbol) -> Self {
        Atom::Sym(x)
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

fn lift(func: Box<Fn(&[Atom]) -> Result<Atom>>) -> Lambda {
    Box::new(move |args| func(args.iter()
            .map(|arg| { if let &Expr::Atom(ref atom) = arg { atom.clone() } else { panic!() } })
            .collect::<Vec<_>>()
            .as_slice()).map(Expr::Atom)
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn call_fn() {
        let func = Box::new(move |atoms: &[Atom]|
            Ok(Atom::Int(atoms.iter().map(|x| if let &Atom::Int(y) = x {y} else { panic!() }).sum())));
        let add = Function::new(Some("add"), lift(func));
        add.call(vec![Expr::Atom(Atom::Int(1)), Expr::Atom(Atom::Int(2))].as_slice());
    }
}
