#![allow(dead_code)]

use super::symbol::Symbol;
use std::fmt;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Atom {
    Bool(bool),
    Int(i64),
    Flt(f64),
    Str(String),
    Sym(Symbol),
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

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Atom::Bool(boolean) => write!(f, "#{}", if boolean { "t" } else { "f" }),
            Atom::Int(int) => write!(f, "{}", int),
            Atom::Flt(flt) => write!(f, "{}", flt),
            Atom::Str(ref string) => write!(f, "\"{}\"", string),
            Atom::Sym(ref sym) => write!(f, "{}", sym.0),
        }
    }
}
