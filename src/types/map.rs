#![allow(dead_code)]

use std::fmt;
use std::collections::HashMap;
use itertools::Itertools;
use super::Expr;
use error::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    Nil,
    Bool(bool),
    Int(i64),
    Str(String),
}

impl Key {
    pub fn try_from(expr: &Expr) -> Result<Self> {
        match *expr {
            Expr::Nil => Ok(Key::Nil),
            Expr::Bool(b) => Ok(Key::Bool(b)),
            Expr::Int(i) => Ok(Key::Int(i)),
            Expr::Str(ref s) => Ok(Key::Str(s.clone())),
            _ => Err(Error::Msg(format!("cannot use as key: {}", expr))),
        }
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Key::Nil => write!(f, "()"),
            Key::Bool(b) => write!(f, "{}", b),
            Key::Int(i) => write!(f, "{}", i),
            Key::Str(ref s) => write!(f, "{}", s),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Map(HashMap<Key, Expr>);

impl Map {
    pub fn new() -> Self {
        Map(HashMap::new())
    }
}

impl Default for Map {
    fn default() -> Self {
        Map(HashMap::new())
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pairs = self.0.iter()
            .map(|(key, val)| format!("{}: {}", key, val))
            .join(", ");
        write!(f, "{{{}}}", pairs)
    }
}
