use error::*;
use ops;

use std::fmt;
use std::iter;
use std::slice;


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
        write!(f, "{}", match *self {
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
    //Str(String),
}

impl Atom {
    /// Return a string denoting the payload data type.
    fn kind(&self) -> &str {
        use self::Atom::*;
        match *self {
            Sym(_) => "sym",
            Int(_) => "int",
        }
    }

    /// Unary negation of an integer.
    pub fn neg(&self) -> Result<Atom> {
        use self::Atom::*;
        match *self {
            Int(x) => Ok(Int(-x)),
            _ => Err(format!(
                "incompatible type for {}: {}",
                Symbol::Sub, self.kind()).into()),
        }
    }

    /// Binary addition of two numbers.
    pub fn add(&self, x: &Self) -> Result<Atom> {
        use self::Atom::*;
        match (*self, *x) {
            (Int(a), Int(b)) => Ok(Int(a + b)),
            _ => Err(format!(
                "incompatible types for {}: {}, {}",
                Symbol::Add, self.kind(), x.kind()).into()),
        }
    }

    /// Binary multiplication of two numbers.
    pub fn sub(&self, x: &Self) -> Result<Atom> {
        use self::Atom::*;
        match (*self, *x) {
            (Int(a), Int(b)) => Ok(Int(a - b)),
            _ => Err(format!(
                "incompatible types for {}: {}, {}",
                Symbol::Sub, self.kind(), x.kind()).into()),
        }
    }

    /// Binary multiplication of two numbers.
    pub fn mul(&self, x: &Self) -> Result<Atom> {
        use self::Atom::*;
        match (*self, *x) {
            (Int(a), Int(b)) => Ok(Int(a * b)),
            _ => Err(format!(
                "incompatible types for {}: {}, {}",
                Symbol::Mul, self.kind(), x.kind()).into()),
        }
    }

    /// Binary division of two integers. Errors on zero division.
    pub fn div(&self, x: &Self) -> Result<Atom> {
        use self::Atom::*;
        match (*self, *x) {
            (Int(a), Int(b)) => {
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

    /// Return the integer remainder when divided by a divisor `x`.
    pub fn modulus(&self, x: &Self) -> Result<Atom> {
        use self::Atom::*;
        match (*self, *x) {
            (Int(a), Int(b)) => {
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

impl From<i32> for Atom {
    fn from(x: i32) -> Self {
        Atom::Int(x)
    }
}

/// The value of an expression, when evaluated.
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

    fn unary<F>(&self, symbol: Symbol, f: F) -> Result<Value>
        where F: Fn(&Atom) -> Result<Atom>
    {
        use self::Value::Atom;
        match *self {
            Atom(ref x) => f(x).map(Value::from),
            _ => Err(format!("incompatible types for {}: {}",
                symbol, self.kind()).into()),
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

    pub fn neg(&self) -> Result<Value> {
        self.unary(Symbol::Sub, Atom::neg)
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

    fn unary_list<F>(&self, symbol: Symbol, f: F) -> Result<Value>
        where F: Fn(&List) -> Result<Value>
    {
        use self::Value::List;
        match *self {
            List(ref v) => f(v),
            _ => Err(format!("incompatible types for {}: {}",
                symbol, self.kind()).into()),
        }
    }

    pub fn head(&self) -> Result<Value> {
        self.unary_list(Symbol::Head, List::head)
    }

    pub fn tail(&self) -> Result<Value> {
        self.unary_list(Symbol::Tail, List::tail)
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

impl<T> From<T> for Value where T: Into<Atom>
{
    fn from(x: T) -> Self {
        Value::Atom(x.into())
    }
}

impl From<List> for Value
{
    fn from(x: List) -> Self {
        Value::List(x)
    }
}

impl iter::Sum<Value> for Value {
    fn sum<I>(iter: I) -> Self where I: Iterator<Item=Value> {
        iter.fold(0.into(), |acc, x| acc.add(&x).unwrap())
    }
}

impl iter::Product<Value> for Value {
    fn product<I>(iter: I) -> Self where I: Iterator<Item=Value> {
        iter.fold(1.into(), |acc, x| acc.mul(&x).unwrap())
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

    /// Return the first element of the List.
    pub fn head(&self) -> Result<Value> {
        self.inner
            .first()
            .ok_or("cannot get head of empty list".into())
            .and_then(Node::eval)
    }

    /// Return all elements excepting the first.
    pub fn tail(&self) -> Result<Value> {
        let output = self.inner
            .split_first()
            .map(|x| x.1)
            .unwrap_or(&[]);

        Ok(Value::List(List::from(output)))
    }
}

impl<'a> From<&'a [Node]> for List {
    fn from(v: &'a [Node]) -> Self {
        List { inner: Vec::from(v) }
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

/// A evaluable list, with the first element `sym` operating upon the
/// other elements `args`.
#[derive(Clone, Debug, PartialEq)]
pub struct Expr {
    sym: Symbol,
    args: Vec<Node>,
}

impl Expr {
    /// Constructs an `Expr`, with function `sym` and a `Vec` of arguments.
    pub fn new(sym: Symbol, args: Vec<Node>) -> Self {
        Expr {
            sym: sym,
            args: args,
        }
    }

    /// Return the function itself.
    pub fn sym(&self) -> Symbol {
        self.sym
    }

    /// Return an iterator over the arguments.
    pub fn args(&self) -> slice::Iter<Node> {
        self.args.iter()
    }

    /// Return the number of arguments.
    pub fn num_args(&self) -> usize {
        self.args.len()
    }

    /// Enforces the correct number of arguments (arity) is used for the
    /// indicated operation.
    fn check_num_args(&self) -> bool {
        use self::Symbol::*;
        let num_args = self.num_args();
        match self.sym() {
            Add | Mul => num_args >= 2,
            Sub => num_args == 1 || num_args == 2,
            Div | Mod => num_args == 2,
            Head | Tail => num_args == 1,
        }
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
    pub fn eval(&self) -> Result<Value> {
        use self::Symbol::*;

        let sym = self.sym();

        // Check if number of arguments match expected number
        if !self.check_num_args() {
            return Err(format!("error: \'{}\' symbol did not expect {} args", sym, self.num_args())
                .into());
        }

        let evaled_args = self.args().map(Node::eval).collect::<Result<Vec<_>>>();

        // Return the first expr that cannot eval correctly
        match evaled_args {
            Err(e) => Err(e),
            Ok(v) => match sym {
                Add => ops::add(&v),
                Sub => ops::sub(&v),
                Mul => ops::mul(&v),
                Div => ops::div(&v),
                Mod => ops::modulus(&v),
                Head => ops::head(&v),
                Tail => ops::tail(&v),
            },
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
    pub fn eval(&self) -> Result<Value> {
        use self::Node::*;
        match *self {
            Atom(a) => Ok(Value::Atom(a)),
            List(ref l) => Ok(Value::List(l.clone())),
            Expr(ref e) => e.eval(),
        }
    }
}

impl<T> From<T> for Node where T: Into<Value>
{
    fn from(x: T) -> Self {
        match x.into() {
            Value::Atom(a) => Node::Atom(a),
            Value::List(l) => Node::List(l),
        }
    }
}

impl From<Expr> for Node
{
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
    fn print_symbol() {
        assert_eq!(Symbol::Add.to_string(),  "+");
        assert_eq!(Symbol::Sub.to_string(),  "-");
        assert_eq!(Symbol::Mul.to_string(),  "*");
        assert_eq!(Symbol::Div.to_string(),  "/");
        assert_eq!(Symbol::Mod.to_string(),  "%");
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
            if let Ok(Atom::Int(i)) = Atom::from(x).neg() {
                i == -x
            } else {
                false
            }
        }

        fn atom_neg_identity(x: i32) -> bool {
            if let Ok(Atom::Int(i)) = Atom::from(x).neg().and_then(|x| x.neg()) {
                x == i
            } else {
                false
            }
        }

        fn atom_add(x: i32, y: i32) -> bool {
            if let Ok(Atom::Int(i)) = Atom::from(x).add(&y.into()) {
                i == x + y
            } else {
                false
            }
        }

        fn atom_sub(x: i32, y: i32) -> bool {
            if let Ok(Atom::Int(i)) = Atom::from(x).sub(&y.into()) {
                i == x - y
            } else {
                false
            }
        }

        fn atom_mul(x: i32, y: i32) -> bool {
            if let Ok(Atom::Int(i)) = Atom::from(x).mul(&y.into()) {
                i == x * y
            } else {
                false
            }
        }

        fn atom_div(x: i32, y: i32) -> bool {
            let result = Atom::from(x).div(&y.into());
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

    // Value tests

    // Expr tests

    // Node tests
}
