use std::fmt;
// use std::iter;
use std::slice;
use std::ops;
use std::collections::HashMap;
use error::*;

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

impl fmt::Display for Arity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Arity::*;
        let text = match *self {
            Nullary => "0",
            Unary   => "1",
            Binary  => "2",
            Ternary => "3",
            Multiary => "at least 2",
        };

        write!(f, "{} args", text)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

impl From<i64> for Atom {
    fn from(x: i64) -> Self {
        Atom::Int(x)
    }
}

impl ops::Add<Atom> for Atom {
    type Output = Atom;
    fn add(self, rhs: Atom) -> Self::Output {
        use self::Atom::*;
        match (self, rhs) {
            (Int(i), Int(j)) => Int(i + j),
            _ => panic!(),
        }
    }
}

impl ops::Sub<Atom> for Atom {
    type Output = Atom;
    fn sub(self, rhs: Atom) -> Self::Output {
        use self::Atom::*;
        match (self, rhs) {
            (Int(i), Int(j)) => Int(i - j),
            _ => panic!(),
        }
    }
}

impl ops::Mul<Atom> for Atom {
    type Output = Atom;
    fn mul(self, rhs: Atom) -> Self::Output {
        use self::Atom::*;
        match (self, rhs) {
            (Int(i), Int(j)) => Int(i * j),
            _ => panic!(),
        }
    }
}

impl ops::Div<Atom> for Atom {
    type Output = Atom;
    fn div(self, rhs: Atom) -> Self::Output {
        use self::Atom::*;
        match (self, rhs) {
            (Int(i), Int(j)) => Int(i / j),
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Atom(Atom),
    List(List),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Value::*;
        match *self {
            Atom(ref t) => write!(f, "{}", t),
            List(ref t) => write!(f, "{}", t),
        }
    }
}

impl<T> From<T> for Value where T: Into<Atom> {
    fn from(v: T) -> Self {
        Value::Atom(v.into())
    }
}

impl ops::Add<Value> for Value {
    type Output = Value;
    fn add(self, rhs: Value) -> Self::Output {
        use self::Value::*;
        match (self, rhs) {
            (Atom(i), Atom(j)) => Atom(i + j),
            _ => panic!(),
        }
    }
}

impl ops::Sub<Value> for Value {
    type Output = Value;
    fn sub(self, rhs: Value) -> Self::Output {
        use self::Value::*;
        match (self, rhs) {
            (Atom(i), Atom(j)) => Atom(i - j),
            _ => panic!(),
        }
    }
}

impl ops::Mul<Value> for Value {
    type Output = Value;
    fn mul(self, rhs: Value) -> Self::Output {
        use self::Value::*;
        match (self, rhs) {
            (Atom(i), Atom(j)) => Atom(i * j),
            _ => panic!(),
        }
    }
}

impl ops::Div<Value> for Value {
    type Output = Value;
    fn div(self, rhs: Value) -> Self::Output {
        use self::Value::*;
        match (self, rhs) {
            (Atom(i), Atom(j)) => Atom(i / j),
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Atom (Atom),
    List (List),
    Expr (Expr),
}

impl Node {
    fn eval(&self) -> Result<Value> {
        use self::Node::*;
        match *self {
            Atom(a) => Ok(Value::Atom(a)),
            List(ref l) => Ok(Value::List(l.clone())),
            Expr(ref e) => e.eval(),
        }
    }
}

impl<T> From<T> for Node where T: Into<Atom> {
    fn from(v: T) -> Self {
        Node::Atom(v.into())
    }
}

// impl<T> From<T> for Node where T: Into<List> {
//     fn from(l: T) -> Self {
//         Node::List(l.into())
//     }
// }
//
// impl<T> From<T> for Node where T: Into<Expr> {
//     fn from(e: T) -> Self {
//         Node::Expr(e.into())
//     }
// }

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Node::*;
        match *self {
            Atom(ref a) => write!(f, "{}", a),
            List(ref l) => write!(f, "{}", l),
            Expr(ref e) => write!(f, "{}", e),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct List {
    inner: Vec<Node>
}

impl List {
    pub fn new() -> Self {
        List { inner: Vec::new() }
    }
}

impl From<Vec<Node>> for List {
    fn from(v: Vec<Node>) -> Self {
        List { inner: v }
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let elements = self.inner.iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ");
        write!(f, "( {} )", elements)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expr {
    op: Operator,
    args: Vec<Node>
}

impl Expr {
    pub fn new(op: Operator, args: Vec<Node>) -> Self {
        Expr { op: op, args: args }
    }

    pub fn op(&self) -> Operator {
        self.op
    }

    pub fn args(&self) -> slice::Iter<Node> {
        self.args.iter()
    }

    pub fn num_args(&self) -> usize {
        self.args.len()
    }

    pub fn expect_arity(&self, arity: Arity) -> bool {
        use self::Arity::*;
        let num_args = self.num_args();
        match arity {
            Nullary  => num_args == 0,
            Unary    => num_args == 1,
            Binary   => num_args == 2,
            Ternary  => num_args == 3,
            Multiary => num_args >= 2,
        }
    }

    pub fn eval(&self) -> Result<Value> {
        // use self::Node::*;
        // use self::Atom::*;
        use self::Operator::*;
        use self::Arity::*;

        // Arity = number of arguments accepted
        let mut arity = HashMap::new();

        arity.insert(Add, Multiary);
        arity.insert(Sub, Binary);
        arity.insert(Mul, Multiary);
        arity.insert(Div, Binary);

        let op = self.op();

        let expected_arity = match arity.get(&op) {
            Some(e) => e,
            None => return Err(format!("operator {} not found", op).into())
        };

        // Check if number of arguments match expected number
        if !self.expect_arity(*expected_arity) {
            return Err(format!(
                "{} operator expected {} arguments, but received {}",
                op, expected_arity, self.num_args()
            ).into())
        }

        let mut args = self.args().map(Node::eval);

        // Return the first expr that cannot eval correctly
        if let Some(err) = args.find(|x| x.is_err()) {
            err
        } else {
            let mut eval_args = args.map(Result::unwrap);
            match op {
                Add => Ok(eval_args.fold(0.into(), |acc, x| acc + x)),
                Sub => {
                    let a = eval_args.next().unwrap();
                    let b = eval_args.next().unwrap();
                    Ok(a - b)
                },
                Mul => Ok(eval_args.fold(1.into(), |acc, x| acc * x)),
                Div => {
                    let a = eval_args.next().unwrap();
                    let b = eval_args.next().unwrap();
                    Ok(a / b)
                }
            }
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let elements = self.args.iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ");
        write!(f, "( {} {} )", self.op, elements)
    }
}
