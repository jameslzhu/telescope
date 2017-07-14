#![allow(dead_code)]

use std::sync::Arc;
use std::fmt;

use super::*;

#[derive(Clone, Debug)]
pub enum Expr {
    Nil,
    Func(Arc<Function>),
    Atom(Atom),
    List(List),
    Vector(Vector),
}

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
            &Expr::Atom(ref a) => {
                match a {
                    &Atom::Bool(b) => b,
                    _ => true,
                }
            }
            _ => true,
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
        let add = env.lookup("+").and_then(|f| f.func().cloned()).expect(
            "Expected #[+] in builtins",
        );

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
