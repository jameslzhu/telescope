#![allow(dead_code)]

use super::*;
use std::fmt;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum Expr {
    Nil,
    Bool(bool),
    Int(i64),
    Flt(f64),
    Str(String),
    Sym(Symbol),
    Func(Arc<Function>),
    Macro(Arc<Macro>),
    List(List),
    Vector(Vector),
    Map(Map),
}

impl Expr {
    pub fn boolean(&self) -> Option<bool> {
        if let Expr::Bool(x) = *self {
            Some(x)
        } else {
            None
        }
    }

    pub fn int(&self) -> Option<i64> {
        if let Expr::Int(x) = *self {
            Some(x)
        } else {
            None
        }
    }

    pub fn flt(&self) -> Option<f64> {
        if let Expr::Flt(x) = *self {
            Some(x)
        } else {
            None
        }
    }

    pub fn str(&self) -> Option<&str> {
        if let Expr::Str(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }
    pub fn sym(&self) -> Option<&Symbol> {
        if let Expr::Sym(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }

    pub fn list(&self) -> Option<&List> {
        if let Expr::List(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }

    pub fn vector(&self) -> Option<&Vector> {
        if let Expr::Vector(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }

    pub fn func(&self) -> Option<Arc<Function>> {
        if let Expr::Func(ref x) = *self {
            Some(x.clone())
        } else {
            None
        }
    }

    pub fn truthiness(&self) -> bool {
        match *self {
            Expr::Nil => false,
            Expr::Bool(b) => b,
            _ => true,
        }
    }

    pub fn is_int(&self) -> bool {
        self.int().is_some()
    }

    pub fn is_flt(&self) -> bool {
        self.flt().is_some()
    }

    pub fn is_num(&self) -> bool {
        match *self {
            Expr::Flt(_) | Expr::Int(_) => true,
            _ => false,
        }
    }

    pub fn map_int<E, F>(self, f: F) -> Expr
    where
        F: FnOnce(i64) -> E,
        E: Into<Expr>
    {
        match self {
            Expr::Int(int) => f(int).into(),
            _ => self,
        }
    }

    pub fn map_flt<E, F>(self, f: F) -> Expr
    where
        F: FnOnce(f64) -> E,
        E: Into<Expr>
    {
        match self {
            Expr::Flt(flt) => f(flt).into(),
            _ => self,
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expr::Nil => write!(f, "()"),
            Expr::Bool(boolean) => write!(f, "#{}", if boolean { "t" } else { "f" }),
            Expr::Int(int) => write!(f, "{}", int),
            Expr::Flt(flt) => write!(f, "{}", flt),
            Expr::Str(ref string) => write!(f, "\"{}\"", string),
            Expr::Sym(ref sym) => write!(f, "{}", sym.0),
            Expr::Func(ref func) => write!(f, "{}", func),
            Expr::Macro(ref mac) => write!(f, "{}", mac),
            Expr::List(ref list) => write!(f, "{}", list),
            Expr::Vector(ref vec) => write!(f, "{}", vec),
            Expr::Map(ref map) => write!(f, "{}", map),
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        use self::Expr::*;
        match (self, other) {
            (&Nil, &Nil) => true,
            (&Bool(ref a), &Bool(ref b)) => a == b,
            (&Int(ref a), &Int(ref b)) => a == b,
            (&Flt(ref a), &Flt(ref b)) => a == b,
            (&Str(ref a), &Str(ref b)) => a == b,
            (&Sym(ref a), &Sym(ref b)) => a == b,
            (&Func(_), &Func(_)) => false,
            (&Macro(_), &Macro(_)) => false,
            (&List(ref a), &List(ref b)) => a == b,
            (&Vector(ref a), &Vector(ref b)) => a == b,
            (&Map(ref a), &Map(ref b)) => a == b,
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use env::Env;
    use ops;

    #[test]
    fn call_fn() {
        let env = ops::env();
        let add = env.lookup("+").clone().and_then(|f| f.func()).expect(
            "Expected #[+] in builtins",
        );

        let nums: Vec<Expr> = vec![1i64, 2i64].into_iter().map(Expr::from).collect();
        let result = add.apply(nums.as_slice(), env.clone());
        assert_eq!(Expr::from(3), result.unwrap());
    }

    #[test]
    fn test_env() {
        let new_scope = Env::default();
        assert!(new_scope.lookup("hello").is_none());
    }
}
