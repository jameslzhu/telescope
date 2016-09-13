use error::*;
use ops;

use std::collections::HashMap;
use std::fmt;
// use std::iter;
use std::slice;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Symbol {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Symbol::*;
        write!(f, "{}", match *self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Mod => "%",
        })
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Arity {
    Nullary,
    Unary,
    Binary,
    Ternary,
    Multiary, // 2+ arguments
}

impl fmt::Display for Arity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Arity::*;
        let text = match *self {
            Nullary => "0",
            Unary => "1",
            Binary => "2",
            Ternary => "3",
            Multiary => "at least 2",
        };

        write!(f, "{}", text)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Atom {
    Sym(Symbol),
    // Bool(bool),
    Int(i64),
    // Float(f64),
    //Str(String),
}

impl Atom {
    fn kind(&self) -> &str {
        use self::Atom::*;
        match *self {
            Sym(_) => "sym",
            Int(_) => "int",
        }
    }

    pub fn add(&self, x: &Self) -> Result<Atom> {
        use self::Atom::*;
        match (self, x) {
            (&Int(a), &Int(b)) => Ok(Int(a + b)),
            _ => Err(format!(
                "incompatible types for {}: {}, {}",
                Symbol::Add, self.kind(), x.kind()).into()),
        }
    }

    pub fn sub(&self, x: &Self) -> Result<Atom> {
        use self::Atom::*;
        match (self, x) {
            (&Int(a), &Int(b)) => Ok(Int(a - b)),
            _ => Err(format!(
                "incompatible types for {}: {}, {}",
                Symbol::Sub, self.kind(), x.kind()).into()),
        }
    }

    pub fn mul(&self, x: &Self) -> Result<Atom> {
        use self::Atom::*;
        match (self, x) {
            (&Int(a), &Int(b)) => Ok(Int(a * b)),
            _ => Err(format!(
                "incompatible types for {}: {}, {}",
                Symbol::Mul, self.kind(), x.kind()).into()),
        }
    }

    pub fn div(&self, x: &Self) -> Result<Atom> {
        use self::Atom::*;
        match (self, x) {
            (&Int(a), &Int(b)) => {
                if b == 0 {
                    Err("division by zero".into())
                } else {
                    Ok(Int(a / b))
                }
            },
            _ => Err(format!(
                "incompatible types for {}: {}, {}",
                Symbol::Div, self.kind(), x.kind()).into()),
        }
    }

    pub fn modulus(&self, x: &Self) -> Result<Atom> {
        use self::Atom::*;
        match (self, x) {
            (&Int(a), &Int(b)) => {
                if b == 0 {
                    Err("modulus by zero".into())
                } else {
                    Ok(Int(a % b))
                }
            },
            _ => Err(format!(
                "incompatible types for {}: {}, {}",
                Symbol::Mod, self.kind(), x.kind()).into()),
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Atom::*;
        match *self {
            Sym(ref t) => write!(f, "{}", t),
            // Bool(ref t) => write!(f, "{}", t),
            Int(ref t) => write!(f, "{}", t),
            // Float(ref t) => write!(f, "{}", t),
            // Str(ref t) => write!(f, "{}", t),
        }
    }
}

impl From<Symbol> for Atom {
    fn from(x: Symbol) -> Self {
        Atom::Sym(x)
    }
}

impl From<i64> for Atom {
    fn from(x: i64) -> Self {
        Atom::Int(x)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Atom(Atom),
    List(List),
}

impl Value {
    fn kind(&self) -> &str {
        use self::Value::*;
        match *self {
            Atom(_) => "atom",
            List(_) => "list",
        }
    }

    fn binary<F>(&self, x: &Self, symbol: Symbol, f: F) -> Result<Value>
        where F: Fn(&Atom, &Atom) -> Result<Atom>
    {
        use self::Value::Atom;
        match (self, x) {
            (&Atom(ref a), &Atom(ref b)) => f(a, b).map(Value::from),
            _ => Err(format!("incompatible types for {}: {}, {}",
                symbol, self.kind(), x.kind()).into()),
        }
    }

    pub fn add(&self, x: &Value) -> Result<Value> {
        self.binary(x, Symbol::Add, Atom::add)
    }

    pub fn sub(&self, x: &Value) -> Result<Value> {
        self.binary(x, Symbol::Sub, Atom::sub)
    }

    pub fn mul(&self, x: &Value) -> Result<Value> {
        self.binary(x, Symbol::Mul, Atom::mul)
    }

    pub fn div(&self, x: &Value) -> Result<Value> {
        self.binary(x, Symbol::Div, Atom::div)
    }

    pub fn modulus(&self, x: &Value) -> Result<Value> {
        self.binary(x, Symbol::Mod, Atom::modulus)
    }
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

impl<T> From<T> for Value
    where T: Into<Atom>
{
    fn from(v: T) -> Self {
        Value::Atom(v.into())
    }
}

// Unstable feature iter_arith_traits - issue #34529
// Stabilized by Rust 1.12
// impl iter::Sum<Value> for Value {
//     fn sum<I>(iter: I) -> Self where I: Iterator<Item=Value> {
//         iter.fold(0.into(), |acc, x| acc + x)
//     }
// }

// Unstable feature iter_arith_traits - issue #34529
// Stabilized by Rust 1.12
// impl iter::Mul<Value> for Value {
//     fn mul<I>(iter: I) -> Self where I: Iterator<Item=Value> {
//         iter.fold(1.into(), |acc, x| acc * x)
//     }
// }

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Atom(Atom),
    List(List),
    Expr(Expr),
}

impl Node {
    pub fn eval(&self) -> Result<Value> {
        use self::Node::*;
        match *self {
            Atom(a) => Ok(Value::Atom(a)),
            List(ref l) => Ok(Value::List(l.clone())),
            Expr(ref e) => e.eval(),
        }
    }
}

impl<T> From<T> for Node
    where T: Into<Atom>
{
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

#[derive(Clone, Default, Debug, PartialEq)]
pub struct List {
    inner: Vec<Node>,
}

impl List {
    pub fn new() -> Self {
        List::default()
    }
}

impl From<Vec<Node>> for List {
    fn from(v: Vec<Node>) -> Self {
        List { inner: v }
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let elements = self.inner
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ");
        write!(f, "[ {} ]", elements)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expr {
    sym: Symbol,
    args: Vec<Node>,
}

impl Expr {
    pub fn new(sym: Symbol, args: Vec<Node>) -> Self {
        Expr {
            sym: sym,
            args: args,
        }
    }

    pub fn sym(&self) -> Symbol {
        self.sym
    }

    pub fn args(&self) -> slice::Iter<Node> {
        self.args.iter()
    }

    pub fn num_args(&self) -> usize {
        self.args.len()
    }

    fn check_arity(&self, arity: Arity) -> bool {
        use self::Arity::*;
        let num_args = self.num_args();
        match arity {
            Nullary => num_args == 0,
            Unary => num_args == 1,
            Binary => num_args == 2,
            Ternary => num_args == 3,
            Multiary => num_args >= 2,
        }
    }

    pub fn eval(&self) -> Result<Value> {
        use self::Symbol::*;
        use self::Arity::*;

        // Arity = number of arguments accepted
        let mut arity = HashMap::new();

        arity.insert(Add, Multiary);
        arity.insert(Sub, Binary);
        arity.insert(Mul, Multiary);
        arity.insert(Div, Binary);
        arity.insert(Mod, Binary);

        let sym = self.sym();

        let expected_arity = match arity.get(&sym) {
            Some(e) => e,
            None => return Err(format!("symbol {} has unknown number of arguments", sym).into()),
        };

        // Check if number of arguments match expected number
        if !self.check_arity(*expected_arity) {
            return Err(format!("{} symbol expected {} arguments, but received {}",
                               sym,
                               expected_arity,
                               self.num_args())
                .into());
        }

        let evaled_args: Result<Vec<_>> = self.args().map(Node::eval).collect();

        // Return the first expr that cannot eval correctly
        match evaled_args {
            Err(e) => Err(e),
            Ok(v) => {
                match sym {
                    Add => ops::add(&v),
                    Sub => ops::sub(&v),
                    Mul => ops::mul(&v),
                    Div => ops::div(&v),
                    Mod => ops::modulus(&v),
                }
            }
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let elements = self.args
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ");
        write!(f, "( {} {} )", self.sym, elements)
    }
}
