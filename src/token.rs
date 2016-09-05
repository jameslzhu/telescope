#![allow(unused_imports)]
#![allow(dead_code)]

use std::fmt;
use std::iter;
use std::collections::HashMap;

use error::Result;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
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

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Arity {
    Nullary,
    Unary,
    Binary,
    Ternary,
    Multiary,  // 2+ arguments
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Atom {
    Op(Operator),
    // Bool(bool),
    Int(i64),
    // Float(f64),
    // Str(String),
}

impl Atom {
    pub fn op(&self) -> Result<&Operator> {
        use self::Atom::*;
        match *self {
            Op(ref x) => Ok(x),
            _ => Err(format!("cannot convert atom {} into op", self).into()),
        }
    }

    pub fn int(&self) -> Result<i64> {
        use self::Atom::*;
        match *self {
            Int(x) => Ok(x),
            _ => Err(format!("cannot convert atom {} into int", self).into()),
        }
    }
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

#[derive(Clone, Debug, PartialEq)]
pub struct Expr {
    inner: Vec<Node>
}

impl Expr {
    pub fn new() -> Self {
        Expr { inner: Vec::new() }
    }

    pub fn op(&self) -> Operator {
        let &Node::Atom(a) = self.inner.first().unwrap();
        let Atom::Op(o) = a;
        // self.inner.first().atom().unwrap().op().unwrap()
        unimplemented!();
    }

    pub fn args(&self) -> &[Expr] {
        unimplemented!();
    }

    pub fn eval(&self) -> Result<Atom> {
        use self::Node::*;
        use self::Atom::*;
        use self::Operator::*;
        use self::Arity::*;

        // Arity = number of arguments accepted
        let mut arity = HashMap::new();

        arity.insert(Add, Multiary);
        arity.insert(Sub, Binary);
        arity.insert(Mul, Multiary);
        arity.insert(Div, Binary);

        let op = self.op();
        let args = self.args();
        let num_args = args.len();

        // Check arity
        match arity.get(op) {
            Nullary  => assert_eq!(num_args, 0),
            Unary    => assert_eq!(num_args, 1),
            Binary   => assert_eq!(num_args, 2),
            Ternary  => assert_eq!(num_args, 3),
            Multiary => assert!(num_args >= 1),
        };

        op.and_then(|o| match *o {
            Add => args.iter().fold(Ok(0), |acc, ref x| {
                let e = try!(x.eval());
                acc.map(|a| a + e)
            }),
            Sub => {
                let a = args[0].eval();
                let b = args[1].eval();
                a.and_then(|x| b.map(|y| x - y))
                // let a = try!(l[1].eval());
                // let b = try!(l[2].eval());
                // Ok(a - b)
            },
            Mul => args.fold(Ok(1), |acc, ref x| {
                let e = try!(x.eval());
                acc.map(|a| a * e)
            }),
            Div => {
                let a = try!(args[0].eval());
                let b = try!(args[1].eval());
                Ok(a / b)
            }
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Atom (Atom),
    Expr (Expr),
}

impl Node {
    pub fn new() -> Self {
        Node::Expr(Expr::new())
    }

    pub fn is_atom(&self) -> bool {
        if let Node::Atom(_) = *self {true} else {false}
    }

    pub fn is_list(&self) -> bool {
        return !self.is_atom();
    }

    pub fn eval(&self) -> Result<Atom> {
        match *self {
            Atom(ref a) => Ok(a),
            Expr(ref e) => e.eval(),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Node::*;
        match *self {
            Atom(ref a) => write!(f, "{}", a),
            Expr(ref l) => {
                let elements = l.inner.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ");
                write!(f, "( {} )", elements)
            },
        }
    }
}

impl<T> From<T> for Node where T: Into<Atom> {
    fn from(v: T) -> Self {
        Node::Atom(v.into())
    }
}

impl<T> From<Vec<T>> for Expr where T: Into<Atom> {
    fn from(v: Vec<T>) -> Self {
        let op = v.first().into::<Atom>().op().expect("Expected first atom to be operator");

        Expr::new(op, v.into_iter().map(Node::from).skip(1).collect())
    }
}

impl iter::FromIterator<Node> for Expr {
    fn from_iter<U>(iterator: U) -> Self
        where U: IntoIterator<Item = Node>
    {
        Expr::new(iterator.into_iter().collect().into())
    }
}

impl<T> iter::FromIterator<T> for Expr where T: Into<Atom>{
    fn from_iter<U>(iterator: U) -> Self
        where U: IntoIterator<Item = T>
    {
        Expr(iterator.into_iter().map(Expr::from).collect())
    }
}
