use error::*;
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

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Atom {
    Nil,
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(pub String);

#[derive(Clone)]
pub struct Function {
    name: Option<String>,
    func: Rc<Lambda>,
}

pub type Lambda = Box<Fn(&[Expr]) -> Result<Expr>>;

impl Expr {
    pub fn eval(&self) -> Result<Expr> {
        if let &Expr::List(ref lst) = self {
            if let Some((first, rest)) = lst.0.split_first() {
                if let &Expr::Func(ref func) = first {
                    return func.apply(rest)
                } else {
                    return Err("expected function call".into())
                }
            }
        }
        Ok(self.clone())
    }

    pub fn atom(self) -> Option<Atom> {
        if let Expr::Atom(x) = self { Some(x) } else { None }
    }

    pub fn list(self) -> Option<List> {
        if let Expr::List(x) = self { Some(x) } else { None }
    }

    pub fn quote(self) -> Option<Quote> {
        if let Expr::Quote(x) = self { Some(x) } else { None }
    }

    pub fn func(self) -> Option<Function> {
        if let Expr::Func(x) = self { Some(x) } else { None }
    }
}

impl Atom {
    pub fn is_nil(&self) -> bool {
        match self {
            &Atom::Nil => true,
            _ => false
        }
    }

    pub fn is_boolean(&self) -> bool {
        self.boolean().is_some()
    }

    pub fn is_int(&self) -> bool {
        self.int().is_some()
    }

    pub fn is_flt(&self) -> bool {
        self.flt().is_some()
    }

    pub fn is_str(&self) -> bool {
        self.str().is_some()
    }

    pub fn is_num(&self) -> bool {
        match self {
            &Atom::Flt(_) | &Atom::Int(_) => true,
            _ => false,
        }
    }

    pub fn boolean(&self) -> Option<bool> {
        if let &Atom::Bool(x) = self { Some(x) } else { None }
    }

    pub fn int(&self) -> Option<i64> {
        if let &Atom::Int(x) = self { Some(x) } else { None }
    }

    pub fn flt(&self) -> Option<f64> {
        if let &Atom::Flt(x) = self { Some(x) } else { None }
    }

    pub fn str(&self) -> Option<&str> {
        if let &Atom::Str(ref x) = self { Some(x) } else { None }
    }

    pub fn map_int<A, F>(self, f: F) -> Atom
        where F: FnOnce(i64) -> A,
              A: Into<Atom>
    {
        match self {
            Atom::Int(int) => f(int).into(),
            _ => self,
        }
    }

    pub fn map_flt<A, F>(self, f: F) -> Atom
        where F: FnOnce(f64) -> A,
              A: Into<Atom>
    {
        match self {
            Atom::Flt(flt) => f(flt).into(),
            _ => self,
        }
    }
}

impl Function {
    pub fn new<S>(name: Option<S>, func: Lambda) -> Self
        where S: Into<String>
    {
        Function {
            name: name.map(Into::into),
            func: Rc::new(func),
        }
    }

    pub fn apply<'a>(&self, args: &'a [Expr]) -> Result<Expr> {
        // Eval all arguments, returning if any errors
        let evaled_args = args.iter()
            .map(Expr::eval)
            .collect::<Result<Vec<_>>>()
            .chain_err(|| "argument eval failed")?;

        // Call function on args
        (self.func)(&evaled_args)
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
            Atom::Nil => write!(f, "()"),
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
        write!(f, "({})", self.0.split_first()
            .map(|(first, rest)| {
                rest.iter().fold(first.to_string(), |mut acc, x| {
                    acc.push_str(&x.to_string());
                    acc
                })
            }).unwrap_or(String::new())
        )
    }
}

impl fmt::Display for Quote {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.split_first()
            .map(|(first, rest)| {
                rest.iter().fold(first.to_string(), |mut acc, x| {
                    acc.push(' ');
                    acc.push_str(&x.to_string());
                    acc
                })
            }).unwrap_or(String::new())
        )
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl<T> From<T> for Expr
    where T: Into<Atom>
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

impl<T> From<T> for Atom
    where T: Into<Literal>
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

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().zip(other.0.iter())
            .all(|(a, b)| a == b)
    }
}

impl PartialEq for Quote {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().zip(other.0.iter())
            .all(|(a, b)| a == b)
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        use self::Expr::*;
        match (self, other) {
            (&Func(ref f), &Func(ref g)) => false,
            (&Atom(ref a), &Atom(ref b)) => a == b,
            (&List(ref a), &List(ref b)) => a == b,
            (&Quote(ref a), &Quote(ref b)) => a == b,
            _ => false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn lift(func: Box<Fn(&[Atom]) -> Result<Atom>>) -> Lambda {
        Box::new(move |args| {
            func(args.iter()
                    .map(|arg| {
                        if let &Expr::Atom(ref atom) = arg {
                            atom.clone()
                        } else {
                            panic!()
                        }
                    })
                    .collect::<Vec<_>>()
                    .as_slice())
                .map(Expr::Atom)
        })
    }

    #[test]
    fn call_fn() {
        let func = Box::new(move |atoms: &[Atom]| {
            Ok(Atom::Int(atoms.iter()
                .map(|x| if let &Atom::Int(y) = x { y } else { panic!() })
                .sum()))
        });
        let add = Function::new(Some("add"), lift(func));
        let result = add.apply(vec![Expr::Atom(Atom::Int(1)), Expr::Atom(Atom::Int(2))].as_slice());
        assert_eq!(Some(Atom::from(3)), result.unwrap().atom());
    }
}
