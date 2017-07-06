use error::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
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
    Builtin {
        name: String,
        func: Rc<Box<Lambda>>
    },
    User {
        params: Vec<Symbol>,
        body: Rc<Expr>
    },
}

pub struct Env<'a> {
    symbols: HashMap<String, Expr>,
    parent: Option<&'a Env<'a>>,
}

pub type Lambda = Fn(&[Expr], &Env) -> Result<Expr>;

impl Expr {
    pub fn eval(&self, env: &Env) -> Result<Expr> {
        match self {
            &Expr::List(ref lst) => {
                if let Some((first, rest)) = lst.0.split_first() {
                    if let Ok(Expr::Func(ref func)) = first.eval(env) {
                        func.apply(rest, env)
                    } else {
                        Err("expected function call".into())
                    }
                } else {
                    Ok(self.clone())
                }
            }
            &Expr::Atom(Atom::Sym(ref symbol)) => {
                env.lookup(&symbol.0).map(Clone::clone).ok_or(
                    format!(
                        "undefined symbol: {}",
                        symbol.0
                    ).into(),
                )
            }
            _ => Ok(self.clone()),
        }
    }

    pub fn atom(self) -> Option<Atom> {
        if let Expr::Atom(x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn list(self) -> Option<List> {
        if let Expr::List(x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn vector(self) -> Option<Vector> {
        if let Expr::Vector(x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn func(self) -> Option<Function> {
        if let Expr::Func(x) = self {
            Some(x)
        } else {
            None
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
    pub fn new<S>(name: S, func: Box<Lambda>) -> Self
    where
        S: Into<String>
    {
        Function::Builtin {
            name: name.into(),
            func: Rc::new(func),
        }
    }

    #[allow(unused_variables)]
    pub fn apply<'a>(&self, args: &'a [Expr], env: &Env) -> Result<Expr> {
        // Eval all arguments, returning if any errors
        let evaled_args = args.iter()
            .map(|a| a.eval(env))
            .collect::<Result<Vec<_>>>()
            .chain_err(|| "argument eval failed")?;

        // Call function on args
        match self {
            &Function::Builtin { ref name, ref func } => (func)(&evaled_args, env),
            &Function::User { ref params, ref body } => {
                if args.len() != params.len() {
                    Err(format!("fn expected {} args", params.len()).into())
                } else {
                    // Create new env with arguments, eval body with new env
                    let bound_params = params.iter()
                        .cloned()
                        .map(|x| x.0)
                        .zip(args.iter().cloned())
                        .collect();
                    body.eval(&Env::new(bound_params, env))
                }
            }
        }
        
    }
}

impl<'a> Env<'a> {
    pub fn new<E>(symbols: HashMap<String, Expr>, parent: E) -> Self
    where
        E: Into<Option<&'a Env<'a>>>,
    {
        Env {
            symbols: symbols,
            parent: parent.into(),
        }
    }

    pub fn lookup(&self, symbol: &str) -> Option<&Expr> {
        self.symbols.get(symbol).or_else(|| {
            self.parent.and_then(|p| p.lookup(symbol))
        })
    }
}

impl fmt::Debug for Function {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Function::Builtin { ref name, ref func } => {
                write!(f, "#[{}]", name)
            },
            &Function::User { ref params, ref body } => {
                write!(f, "(fn [{}] {})", params.iter().join(" "), body)
            }
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
    use ops;

    fn lift(func: Box<Fn(&[Atom], &Env) -> Result<Atom>>) -> Box<Lambda> {
        Box::new(move |args, env| {
            func(
                args.iter()
                    .map(|arg| if let &Expr::Atom(ref atom) = arg {
                        atom.clone()
                    } else {
                        panic!()
                    })
                    .collect::<Vec<_>>()
                    .as_slice(),
                env,
            ).map(Expr::Atom)
        })
    }

    #[test]
    fn call_fn() {
        let env = ops::env();
        let func = Box::new(move |atoms: &[Atom], _env: &Env| {
            Ok(Atom::Int(
                atoms
                    .iter()
                    .map(|x| if let &Atom::Int(y) = x { y } else { panic!() })
                    .sum(),
            ))
        });
        let add = Function::new("+", lift(func));

        let nums: Vec<Expr> = vec![1i64, 2i64].into_iter().map(Expr::from).collect();
        let result = add.apply(nums.as_slice(), &env);
        assert_eq!(Some(Atom::from(3)), result.unwrap().atom());
    }

    #[test]
    fn test_env() {
        let global_scope = ops::env();
        let new_scope = Env::new(HashMap::new(), &global_scope);
        assert!(new_scope.lookup("hello").is_none());
    }
}
