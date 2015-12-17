#![allow(dead_code)]

use std::fmt;
use std::slice;
use std::vec;
use std::iter::FromIterator;

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

#[derive(Clone, Debug)]
pub enum Node<T> {
    Num(T),
    Sym(Sym),
    List(List<T>),
}

impl<T> PartialEq for Node<T> where T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        use self::Node::*;

        match (self, other) {
            (&Num(ref a), &Num(ref b)) => a == b,
            (&Sym(ref a), &Sym(ref b)) => a == b,
            (&List(ref a), &List(ref b)) => a == b,
            _ => false,
        }
    }
}

impl<T> fmt::Display for Node<T> where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Node::*;
        match *self {
            Num(ref t) => write!(f, "{}", t),
            Sym(ref t) => write!(f, "{}", t),
            List(ref t) => write!(f, "({})", t),
        }
    }
}

#[derive(Clone, Debug)]
pub struct List<T> {
    pub elems: Vec<Node<T>>,
}

pub type Iter<'a, T> = slice::Iter<'a, Node<T>>;
pub type IterMut<'a, T> = slice::IterMut<'a, Node<T>>;
pub type IntoIter<T> = vec::IntoIter<Node<T>>;

impl<T> List<T> {
    pub fn new() -> Self {
        List::<T>::default()
    }

    pub fn iter(&self) -> Iter<T> {
        self.elems.iter()
    }
}

impl<T> PartialEq for List<T> where T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.elems.iter().zip(other.elems.iter()).all(|(a, b)| a.eq(&b))
    }
}

impl<T> fmt::Display for List<T> where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "( {} )",
               self.elems.iter().fold(String::from(""), |a, ref e| format!("{} {}", a, e)))
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        List::<T> { elems: Vec::<_>::new() }
    }
}

// impl<T> FromIterator<T> for List<T> {
//     fn from_iter<U>(iterator: U) -> Self where U : IntoIterator<Item=T> {
//         List::<T> {
//             elems : Vec::from_iter(iterator.into_iter().map(|e| Node::Num(e)))
//         }
//     }
// }

impl<T> FromIterator<Node<T>> for List<T> {
    fn from_iter<U>(iterator: U) -> Self
        where U: IntoIterator<Item = Node<T>>
    {
        List::<T> { elems: Vec::from_iter(iterator.into_iter()) }
    }
}

impl<T> IntoIterator for List<T> {
    type Item = Node<T>;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        self.elems.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a Node<T>;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.elems.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut List<T> {
    type Item = &'a mut Node<T>;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(mut self) -> IterMut<'a, T> {
        self.elems.iter_mut()
    }
}

impl<T> From<Vec<Node<T>>> for List<T> {
    fn from(v: Vec<Node<T>>) -> Self {
        List::<T> { elems: v }
    }
}
