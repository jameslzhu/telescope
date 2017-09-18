use std::collections::HashMap;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

use types::{Expr, Symbol};

#[derive(Clone, Debug)]
struct EnvImpl {
    symbols: HashMap<String, Expr>,
    parent: Option<Env>,
}

#[derive(Clone, Debug)]
pub struct Env(Rc<RefCell<EnvImpl>>);

impl Env {
    pub fn new(symbols: HashMap<String, Expr>, parent: Option<Env>) -> Self {
        Env( Rc::new( RefCell::new( EnvImpl {
            symbols: symbols,
            parent: parent,
        })))
    }

    pub fn lookup(&self, symbol: &str) -> Option<Expr> {
        let borrowed: Ref<EnvImpl> = (*self.0).borrow();
        let self_lookup = borrowed.symbols.get(symbol).cloned();
        match self_lookup {
            Some(value) => Some(value),
            None => match borrowed.parent.clone() {
                Some(parent) => parent.lookup(symbol),
                None => None,
            }
        }
    }

    pub fn define(&self, symbol: &str, value: Expr) -> Symbol {
        (*self.0).borrow_mut().symbols.insert(symbol.to_string(), value);
        Symbol(symbol.to_string())
    }
}

impl Default for Env {
    fn default() -> Self {
        Env::new(HashMap::default(), None)
    }
}
