#![allow(unused_imports)]
#![allow(dead_code)]

use std::fmt;
use std::iter;

use error::Result;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    pub fn parse(x: &str) -> Result<Self> {
        use self::Operator::*;
        match x {
            "+" => Ok(Add),
            "-" => Ok(Sub),
            "*" => Ok(Mul),
            "/" => Ok(Div),
            _ => Err(format!("could not parse operator {}", x).into()),
        }
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Operator::*;
        write!(f, "{}", match *self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Atom {
    Op(Operator),
    // Bool(bool),
    Int(i64),
    // Float(f64),
    // Str(String),
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Atom::*;
        match *self {
            Op(ref t) => write!(f, "{}", t),
            // Bool(ref t) => write!(f, "{}", t),
            Int(ref t) => write!(f, "{}", t),
            // Float(ref t) => write!(f, "{}", t),
            // Str(ref t) => write!(f, "{}", t),
        }
    }
}

impl From<Operator> for Atom {
    fn from(x: Operator) -> Self {
        Atom::Op(x)
    }
}

// impl From<bool> for Atom {
//     fn from(x: bool) -> Self {
//         Atom::Bool(x)
//     }
// }

impl From<i64> for Atom {
    fn from(x: i64) -> Self {
        Atom::Int(x)
    }
}

// impl From<f64> for Atom {
//     fn from(x: f64) -> Self {
//         Atom::Float(x)
//     }
// }
//
// impl From<String> for Atom {
//     fn from(x: String) -> Self {
//         Atom::Str(x.into())
//     }
// }

#[derive(Debug, PartialEq)]
pub enum Expr {
    Atom(Atom),
    List(Vec<Expr>),
}

impl Expr {
    pub fn eval(&self) -> Result<i64> {
        use self::Expr::*;
        use self::Atom::*;
        use self::Operator::*;
        match *self {
            Atom(ref a) => {
                match *a {
                    Op(_) => Err("cannot evaluate operator".into()),
                    Int(n) => Ok(n),
                }
            }
            List(ref l) => {
                let mut iter = l.iter();
                let op_expr = try!(iter.next().ok_or("expected operator as first element"));
                if let &Atom(ref a) = op_expr {
                    if let &Op(ref o) = a {
                        match *o {
                            Add => iter.fold(Ok(0), |acc, ref x| {
                                let e = try!(x.eval());
                                acc.map(|a| a + e)
                            }),
                            Sub => {
                                if l.len() == 3 {
                                    let a = try!(l[1].eval());
                                    let b = try!(l[2].eval());
                                    Ok(a - b)
                                } else {
                                    Err("expected two operands to '-' operation".into())
                                }
                            },
                            Mul => iter.fold(Ok(1), |acc, ref x| {
                                let e = try!(x.eval());
                                acc.map(|a| a * e)
                            }),
                            Div => {
                                if l.len() == 3 {
                                    let a = try!(l[1].eval());
                                    let b = try!(l[2].eval());
                                    Ok(a / b)
                                } else {
                                    Err("expected two operands to '/' operation".into())
                                }
                            }
                        }
                    } else {
                        Err("expected first element to be operator, not number".into())
                    }
                } else {
                    Err("expected first element to be operator, not list".into())
                }
            }
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Expr::*;
        match *self {
            Atom(ref a) => write!(f, "{}", a),
            List(ref l) => {
                let elements = l.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ");
                write!(f, "( {} )", elements)
            },
        }
    }
}

impl<T> From<T> for Expr where T: Into<Atom> {
    fn from(v: T) -> Self {
        Expr::Atom(v.into())
    }
}

impl<T> From<Vec<T>> for Expr where T: Into<Atom> {
    fn from(v: Vec<T>) -> Self {
        Expr::List(v.into_iter().map(Expr::from).collect())
    }
}

impl iter::FromIterator<Expr> for Expr {
    fn from_iter<U>(iterator: U) -> Self
        where U: IntoIterator<Item = Expr>
    {
        Expr::List(iterator.into_iter().collect())
    }
}

impl<T> iter::FromIterator<T> for Expr where T: Into<Atom>{
    fn from_iter<U>(iterator: U) -> Self
        where U: IntoIterator<Item = T>
    {
        Expr::List(iterator.into_iter().map(Expr::from).collect())
    }
}
