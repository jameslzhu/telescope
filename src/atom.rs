use error::*;

use std::vec;
use std::fmt;

/// Represents an object, notably functions and variables.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Symbol {
    /// Addition symbol
    Add,
    /// Subtraction symbol
    Sub,
    /// Multiplication symbol
    Mul,
    /// Division symbol
    Div,
    /// Moduluo (remainder) symbol
    Mod,
    /// Head symbol (see `List::head`)
    Head,
    /// Tail symbol (see `List::tail`)
    Tail,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Symbol::*;
        f.write_str(match *self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Mod => "%",
            Head => "head",
            Tail => "tail",
        })
    }
}

/// Fundamental data type.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Atom {
    Sym(Symbol),
    // Bool(bool),
    Int(i32), 
    // Float(f64),
    // Str(String),
}

impl Atom {
    /// Unary negation of an integer.
    fn neg<I>(iter: I) -> Result<Atom>
        where I: Iterator<Item = Atom>
    {
        iter.last()
            .ok_or("expected one argument to [-]".into())
            .and_then(|n| match n {
                Atom::Int(x) => Ok(Atom::Int(-x)),
                e @ _ => Err(format!("incompatible type for [-]: {}", e).into()),
            })
    }

    fn fold<I, F>(iter: I, init: Atom, sym: Symbol, f: F) -> Result<Atom>
        where I: Iterator<Item = Atom>,
              F: Fn(i32, i32) -> i32
    {
        iter.fold(Ok(init), |acc, x| {
            acc.and_then(|total| match (total, x) {
                (Atom::Int(a), Atom::Int(b)) => Ok(f(a, b).into()),
                e @ _ => Err(format!("incompatible type for [{}]: {}", sym, e.1).into()),
            })
        })
    }

    fn fold_flat<I, F>(iter: I, init: Atom, sym: Symbol, f: F) -> Result<Atom>
        where I: Iterator<Item = Atom>,
              F: Fn(i32, i32) -> Result<i32>
    {
        iter.fold(Ok(init), |acc, x| {
            acc.and_then(|total| match (total, x) {
                (Atom::Int(a), Atom::Int(b)) => f(a, b).map(Into::into),
                e @ _ => Err(format!("incompatible type for [{}]: {}", sym, e.1).into())
            })
        })
    }

    fn add<I>(iter: I) -> Result<Atom>
        where I: Iterator<Item = Atom>
    {
        Atom::fold(iter, 0.into(), Symbol::Add, |a, b| a + b)
    }

    fn sub<I>(mut iter: I) -> Result<Atom>
        where I: Iterator<Item = Atom>
    {
        let first = iter.next().unwrap();
        Atom::fold(iter, first, Symbol::Sub, |a, b| a - b)
    }

    fn mul<I>(iter: I) -> Result<Atom>
        where I: Iterator<Item = Atom>
    {
        Atom::fold(iter, 1.into(), Symbol::Mul, |a, b| a * b)
    }

    fn div<I>(mut iter: I) -> Result<Atom>
        where I: Iterator<Item = Atom>
    {
        let first = iter.next().unwrap();
        Atom::fold_flat(iter, first, Symbol::Div,
                        |a, b| if b != 0 { Ok(a / b) } else { Err("division by zero".into()) })
    }

    /// Return the integer remainder when divided by a divisor `x`.
    pub fn modulus(&self, x: &Self) -> Result<Atom> {
        use self::Atom::*;
        match (*self, *x) {
            (Int(a), Int(b)) => {
                if b != 0 {
                    Ok(Int(a % b))
                } else {
                    Err("modulus by zero".into())
                }
            },
            _ => Err(format!("incompatible types for [%]: {}, {}", self, x).into()),
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

impl From<i32> for Atom {
    fn from(x: i32) -> Self {
        Atom::Int(x)
    }
}

/// A (possibly recursive) collection of any value (Atom, Expr, or itself).
#[derive(Clone, Default, Debug, PartialEq)]
pub struct List {
    inner: Vec<Node>,
}

impl List {
    /// Construct an empty List.
    pub fn new() -> Self {
        List::default()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Remove and return the first element.
    pub fn head(mut self) -> Result<Node> {
        if !self.is_empty() {
            Ok(self.inner.remove(0))
        } else {
            Err("cannot get head of empty list".into())
        }
    }

    /// Remove the first element and return the rest.
    pub fn tail(mut self) -> Result<Node> {
        if !self.is_empty() {
            self.inner.remove(0);
        }
        Ok(self.into())
    }
}

impl<'a> From<&'a [Node]> for List {
    fn from(v: &'a [Node]) -> Self {
        List { inner: Vec::from(v) }
    }
}

impl From<Expr> for List {
    fn from(e: Expr) -> Self {
        List { inner: e.nodes }
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let elements = self.inner
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ");
        write!(f, "[{}]", elements)
    }
}

/// A evaluable list, with the first element `sym` operating upon the
/// other elements `args`.
#[derive(Clone, Debug, PartialEq)]
pub struct Expr {
    nodes: Vec<Node>,
}

impl Expr {
    /// Constructs an `Expr`, with a `Vec` of `Nodes`.
    pub fn new(nodes: Vec<Node>) -> Self {
        Expr { nodes: nodes }
    }

    pub fn num_args(&self) -> usize {
        use std::cmp;
        cmp::max(0, self.nodes.len() - 1)
    }

    /// Evaluate the expression recursively, first evaluating its arguments
    /// before evaluating itself.
    ///
    /// # Errors
    /// `eval` returns an `Error` if any of the following occur (in
    /// descending precedence):
    /// * The number of arguments is incorrect
    /// * Any argument has the wrong type
    /// * Any argument evaluates to an `Error`
    /// * The function itself returns an error
    pub fn eval(&self) -> Result<Node> {
        use self::Symbol::*;

        // Get symbol
        if let Some((symbol, args)) = self.nodes.split_first() {
            if let &Node::Atom(Atom::Sym(sym)) = symbol {
                args.iter()
                    .map(Node::eval)
                    .collect::<Result<Vec<_>>>()
                    .and_then(|mut v| match sym {
                        Add => Node::add(v.into_iter()),
                        Sub => {
                            if self.num_args() == 1 {
                                Node::neg(v.into_iter())    
                            } else {
                                Node::sub(v.into_iter())
                            }
                        },
                        Mul => Node::mul(v.into_iter()),
                        Div => Node::div(v.into_iter()),
                        Mod => Node::modulus(v.into_iter()),
                        Head => Node::head(v.pop().unwrap()),
                        Tail => Node::tail(v.pop().unwrap()),
                    })
            } else {
                Err(format!("undefined symbol {}", symbol).into())
            }
        } else {
            Err("cannot evaluate empty expr".into())
        }
    }
}

impl From<List> for Expr {
    fn from(l: List) -> Self {
        Expr::new(l.inner)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let elements = self.nodes
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ");
        write!(f, "({})", elements)
    }
}

/// The recursive element type of `List` and `Expr`.
#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Atom(Atom),
    List(List),
    Expr(Expr),
}

impl Node {
    /// Evaluate the inner value.
    ///
    /// If the node is an atom or list, it evaluates to itself.
    /// If the node is an expression, it returns the result of evaluating
    /// the expression.
    pub fn eval(&self) -> Result<Node> {
        match *self {
            Node::Expr(ref e) => e.eval(),
            _ => Ok(self.clone())
        }
    }

    pub fn lift<I, F, G>(iter: I, f: F, fn_args: G) -> Result<Node>
        where I: Iterator<Item = Node>,
              F: Fn(vec::IntoIter<Atom>) -> Result<Atom>,
              G: Fn(&[Atom]) -> bool,
    {
        iter.map(|x| match x {
                Node::Atom(y) => Ok(y),
                _ => Err(format!("incompatible types for [+]: {}", x).into())
            })
            .collect::<Result<Vec<Atom>>>()
            .and_then(|a| if fn_args(&a) { Ok(a) } else { Err("unexpected number of arguments".into()) })
            .and_then(|a| f(a.into_iter()))
            .map(Node::from)
    }

    pub fn neg<I>(iter: I) -> Result<Node>
        where I: Iterator<Item = Node>
    {
        Node::lift(iter, Atom::neg, |v| v.len() == 1)
    }

    pub fn add<I>(iter: I) -> Result<Node>
        where I: Iterator<Item = Node>
    {
        Node::lift(iter, Atom::add, |_| true)
    }

    pub fn sub<I>(iter: I) -> Result<Node>
        where I: Iterator<Item = Node>
    {
        Node::lift(iter, Atom::sub, |v| v.len() >= 2)
    }

    pub fn mul<I>(iter: I) -> Result<Node>
        where I: Iterator<Item = Node>
    {
        Node::lift(iter, Atom::mul, |_| true)
    }

    pub fn div<I>(iter: I) -> Result<Node>
        where I: Iterator<Item = Node>
    {
        Node::lift(iter, Atom::div, |v| v.len() >= 2)
    }

    pub fn modulus<I>(iter: I) -> Result<Node>
        where I: Iterator<Item = Node>
    {
        iter.map(|x| match x {
                Node::Atom(y) => Ok(y),
                _ => Err(format!("incompatible types for [%]: {}", x).into())
            })
            .collect::<Result<Vec<Atom>>>()
            .and_then(|a| if a.len() == 2 { Ok(a) } else { Err("unexpected number of arguments".into()) })
            .and_then(|a| Atom::modulus(&a[0], &a[1]))
            .map(Node::from)
    }

    fn lift_list<F>(self, symbol: Symbol, f: F) -> Result<Node>
        where F: Fn(List) -> Result<Node>
    {
        match self {
            Node::List(v) => f(v).map(Node::from),
            _ => Err(format!("incompatible types for {}: {}", symbol, self).into()),
        }
    }

    pub fn head(self) -> Result<Node> {
        self.lift_list(Symbol::Head, List::head)
    }

    pub fn tail(self) -> Result<Node> {
        self.lift_list(Symbol::Tail, List::tail)
    }
}

impl<T> From<T> for Node
    where T: Into<Atom>
{
    fn from(x: T) -> Self {
        Node::Atom(x.into())
    }
}

impl From<List> for Node
{
    fn from(x: List) -> Self {
        Node::List(x)
    }
}

impl From<Expr> for Node {
    fn from(x: Expr) -> Self {
        Node::Expr(x)
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    // Printing tests
    #[test]
    fn print_symbol() {
        assert_eq!(Symbol::Add.to_string(), "+");
        assert_eq!(Symbol::Sub.to_string(), "-");
        assert_eq!(Symbol::Mul.to_string(), "*");
        assert_eq!(Symbol::Div.to_string(), "/");
        assert_eq!(Symbol::Mod.to_string(), "%");
        assert_eq!(Symbol::Head.to_string(), "head");
        assert_eq!(Symbol::Tail.to_string(), "tail");
    }

    quickcheck! {
        fn print_atom_int(xs: i32) -> bool {
            Atom::from(xs).to_string() == xs.to_string()
        }
    }

    // Symbol tests

    // Atom tests
    quickcheck!{
        fn atom_neg(x: i32) -> bool {
            use std::iter::once;
            if let Ok(Atom::Int(i)) = Atom::neg(once(Atom::from(x))) {
                i == -x
            } else {
                false
            }
        }

        fn atom_neg_identity(x: i32) -> bool {
            use std::iter::once;
            if let Ok(Atom::Int(i)) = Atom::neg(once(Atom::from(x))).and_then(|x| Atom::neg(once(x))) {
                x == i
            } else {
                false
            }
        }

        fn atom_add(x: i32, y: i32) -> bool {
            if let Ok(Atom::Int(i)) = Atom::add(vec![Atom::from(x), Atom::from(y)].into_iter()) {
                i == x + y
            } else {
                false
            }
        }

        fn atom_sub(x: i32, y: i32) -> bool {
            if let Ok(Atom::Int(i)) = Atom::sub(vec![Atom::from(x), Atom::from(y)].into_iter()) {
                i == x - y
            } else {
                false
            }
        }

        fn atom_mul(x: i32, y: i32) -> bool {
            if let Ok(Atom::Int(i)) = Atom::mul(vec![Atom::from(x), Atom::from(y)].into_iter()) {
                i == x * y
            } else {
                false
            }
        }

        fn atom_div(x: i32, y: i32) -> bool {
            let result = Atom::div(vec![Atom::from(x), Atom::from(y)].into_iter());
            if y == 0 {
                result.is_err()
            } else if let Ok(Atom::Int(i)) = result {
                i == x / y
            } else {
                false
            }
        }
    }

    // List tests

    // Expr tests

    // Node tests
}
