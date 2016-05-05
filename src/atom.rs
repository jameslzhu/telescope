#![allow(dead_code)]

use std::fmt;
use std::iter;
use std::borrow::Borrow;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Sym {
    Add,
    Sub,
    Mul,
    Div,
}

impl fmt::Display for Sym {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Sym::*;
        write!(f,
               "{}",
               match *self {
                   Add => "+",
                   Sub => "-",
                   Mul => "*",
                   Div => "/",
               })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Atom<T> {
    Num(T),
    Sym(Sym),
}

impl<T> From<Sym> for Atom<T> {
    fn from(s: Sym) -> Self {
        Atom::Sym(s)
    }
}

impl<T> fmt::Display for Atom<T>
    where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Atom::*;
        match *self {
            Num(ref t) => write!(f, "{}", t),
            Sym(ref t) => write!(f, "{}", t),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expr<T> {
    Atom(Atom<T>),
    List(Vec<Expr<T>>),
}

impl<T> Expr<T> {
    fn is_atom(&self) -> bool {
        if let Expr::Atom(_) = *self {
            true
        } else {
            false
        }
    }

    fn is_list(&self) -> bool {
        if let Expr::List(_) = *self {
            true
        } else {
            false
        }
    }
}

impl<T> From<T> for Expr<T> {
    fn from(t: T) -> Self {
        Expr::Atom(Atom::Num(t))
    }
}

impl<T> From<Vec<T>> for Expr<T> {
    fn from(v: Vec<T>) -> Self {
        Expr::List(v.into_iter().map(Expr::from).collect())
    }
}

impl<T> Borrow<[Expr<T>]> for Expr<T>
{
    fn borrow(&self) -> &[Expr<T>] {
        use self::Expr::*;
        match *self {
            Atom(ref a) => { let v = Vec::new(); v.push(Expr::from(*a.clone())); v.as_slice() },
            List(ref l) => l.as_slice(),
        }
    }
}

impl<T> fmt::Display for Expr<T>
    where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Expr::*;
        match *self {
            Atom(ref a) => write!(f, "{}", a),
            List(ref l) => write!(f, "( {} )", l.join(" ")),
        }
    }
}

impl<T> iter::FromIterator<Expr<T>> for Expr<T> {
    fn from_iter<U>(iterator: U) -> Self
        where U: IntoIterator<Item = Expr<T>>
    {
        Expr::List::<T>(iterator.into_iter().collect())
    }
}

impl<T> iter::FromIterator<Atom<T>> for Expr<T> {
    fn from_iter<U>(iterator: U) -> Self
        where U: IntoIterator<Item = Atom<T>>
    {
        Expr::List::<T>(iterator.into_iter().map(Expr::Atom).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!("1", Expr::from(1).to_string());
        assert_eq!("( 1 2 )", Expr::<i32>::from(vec![1, 2]).to_string());
    }
}
