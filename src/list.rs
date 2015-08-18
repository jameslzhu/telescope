#![allow(dead_code)]

use std::fmt;
use std::slice;

#[derive(Copy, Clone, Debug)]
pub enum Sym {
    Add,
    Sub,
    Mul,
    Div,
}

impl fmt::Display for Sym {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            &Sym::Add => "+",
            &Sym::Sub => "-",
            &Sym::Mul => "*",
            &Sym::Div => "/"
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Debug)]
pub enum Node<T> {
    Num(T),
    Sym(Sym),
    List(List<T>),
}

impl<T> Clone for Node<T> where T : Clone {
    fn clone(&self) -> Self {
        match self {
            &Node::Num(ref t) => Node::Num(t.clone()),
            &Node::Sym(ref t) => Node::Sym(t.clone()),
            &Node::List(ref t) => Node::List(t.clone())
        }
    }
}

impl<T> fmt::Display for Node<T> where T : fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Node::Num(ref t) => write!(f, "{}", t),
            &Node::Sym(ref t) => write!(f, "{}", t),
            &Node::List(ref t) => write!(f, "({})", t),
        }
    }
}

#[derive(Debug)]
pub struct List<T> {
    pub elems : Vec<Node<T>>
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List::<T> {
            elems : Vec::<_>::new()
        }
    }

    pub fn iter(&self) -> Iter<T> {
        self.elems.iter()
    }
}

impl<T> Clone for List<T> where T : Clone {
    fn clone(&self) -> Self {
        List::<T> {
            elems : self.elems.clone()
        }
    }
}

impl<T> fmt::Display for List<T> where T : fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( ");
        for e in &self.elems {
            write!(f, "{} ", e);
        }
        write!(f, ")")
    }
}

pub type Iter<'a, T> = slice::Iter<'a, Node<T>>;
