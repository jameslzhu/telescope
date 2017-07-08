use error::*;
use itertools::Itertools;
use std::fmt;
use std::rc::Rc;
use eval::Env;
use token::Literal;

#[derive(Clone, Debug)]
pub enum Expr {
    Nil,
    Func(Function),
    Atom(Atom),
    List(List),
    Vector(Vector),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
pub struct Vector(pub Vec<Expr>);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(pub String);

#[derive(Clone)]
pub enum Function {
    Builtin { name: String, func: Rc<Box<Lambda>> },
    User { name: Option<String>, params: Vec<Symbol>, body: Vec<Expr> },
}

pub type Lambda = Fn(&[Expr], &Env) -> Result<Expr>;

impl Expr {
    pub fn atom(&self) -> Option<&Atom> {
        if let &Expr::Atom(ref x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn list(&self) -> Option<&List> {
        if let &Expr::List(ref x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn vector(&self) -> Option<&Vector> {
        if let &Expr::Vector(ref x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn func(&self) -> Option<&Function> {
        if let &Expr::Func(ref x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn sym(&self) -> Option<&Symbol> {
        self.atom().and_then(|a| a.sym())
    }

    pub fn truthiness(&self) -> bool {
        match self {
            &Expr::Nil => false,
            &Expr::Atom(ref a) => match a {
                &Atom::Bool(b) => b,
                _ => true,
            },
            _ => true,
        }
    }
}

impl Atom {
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
        if let &Atom::Bool(x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn int(&self) -> Option<i64> {
        if let &Atom::Int(x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn flt(&self) -> Option<f64> {
        if let &Atom::Flt(x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn str(&self) -> Option<&str> {
        if let &Atom::Str(ref x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn sym(&self) -> Option<&Symbol> {
        if let &Atom::Sym(ref x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn map_int<A, F>(self, f: F) -> Atom
    where
        F: FnOnce(i64) -> A,
        A: Into<Atom>,
    {
        match self {
            Atom::Int(int) => f(int).into(),
            _ => self,
        }
    }

    pub fn map_flt<A, F>(self, f: F) -> Atom
    where
        F: FnOnce(f64) -> A,
        A: Into<Atom>,
    {
        match self {
            Atom::Flt(flt) => f(flt).into(),
            _ => self,
        }
    }
}

impl Function {
    pub fn builtin<S>(name: S, func: Box<Lambda>) -> Self
    where
        S: Into<String>,
    {
        Function::Builtin {
            name: name.into(),
            func: Rc::new(func),
        }
    }
}

impl fmt::Debug for Function {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Function::Builtin { ref name, ref func }
                => write!(f, "#[{}]", name),
            &Function::User { ref name, ref params, ref body, }
                => write!(f, "(fn [{}] {})", params.iter().join(" "), body.iter().join("\n")),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expr::Nil => write!(f, "()"),
            Expr::Func(ref func) => write!(f, "{}", func),
            Expr::Atom(ref atom) => write!(f, "{}", atom),
            Expr::List(ref list) => write!(f, "{}", list),
            Expr::Vector(ref vec) => write!(f, "{}", vec),
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
        write!(f, "({})", self.0.iter().join(" "))
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.iter().join(" "))
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
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
        Expr::Func(x)
    }
}

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

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        self.0.len() == other.0.len() && self.0.iter().zip(&other.0).all(|(a, b)| a == b)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.0.len() == other.0.len() && self.0.iter().zip(&other.0).all(|(a, b)| a == b)
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        use self::Expr::*;
        match (self, other) {
            (&Nil, &Nil) => true,
            (&Func(_), &Func(_)) => false,
            (&Atom(ref a), &Atom(ref b)) => a == b,
            (&List(ref a), &List(ref b)) => a == b,
            (&Vector(ref a), &Vector(ref b)) => a == b,
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use eval::Env;
    use ops;

    #[test]
    fn call_fn() {
        let mut env = ops::env();
        let add = env.lookup("+")
            .and_then(|f| f.func().cloned())
            .expect("Expected #[+] in builtins");

        let nums: Vec<Expr> = vec![1i64, 2i64].into_iter().map(Expr::from).collect();
        let result = add.apply(nums.as_slice(), &mut env);
        assert_eq!(Expr::from(3), result.unwrap());
    }

    #[test]
    fn test_env() {
        let new_scope = Env::default();
        assert!(new_scope.lookup("hello").is_none());
    }
}
