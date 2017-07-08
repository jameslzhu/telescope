use std::collections::HashMap;
use ast::*;
use error::*;
use forms;

pub struct Env<'a> {
    symbols: HashMap<String, Expr>,
    parent: Option<&'a Env<'a>>,
}

impl Expr {
    pub fn eval(&self, mut env: &mut Env) -> Result<Expr> {
        match self {
            &Expr::List(ref lst) => lst.eval(&mut env),
            &Expr::Atom(ref a) => a.eval(&mut env),
            _ => Ok(self.clone()),
        }
    }
}

impl Atom {
    pub fn eval(&self, env: &mut Env) -> Result<Expr> {
        match self {
            &Atom::Sym(ref symbol) => {
                env.lookup(&symbol.0).cloned()
                    .ok_or(format!("undefined symbol: {}", symbol.0).into())
            },
            _ => Ok(Expr::Atom(self.clone()))
        }
    }
}

impl List {
    pub fn eval(&self, env: &mut Env) -> Result<Expr> {
        if let Some((first, rest)) = self.0.split_first() {
            if let Some(ref sym) = first.sym() {
                if forms::is_special_form(sym) {
                    forms::eval(sym, rest, env)
                } else if let Ok(Expr::Func(ref func)) = first.eval(env) {
                    func.apply(rest, env)
                } else {
                    Err(format!("could not find symbol {}", first).into())
                }
            } else {
                Err("expected function call".into())
            }
        } else {
            Ok(Expr::List(self.clone()))
        }
    }
}

impl Function {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    #[allow(unused_variables)]
    pub fn apply<'a>(&self, args: &'a [Expr], env: &mut Env) -> Result<Expr> {
        // Eval all arguments, returning if any errors
        let evaled_args = args.iter()
            .map(|a| a.eval(env))
            .collect::<Result<Vec<_>>>()
            .chain_err(|| "argument eval failed")?;

        // Call function on args
        match self {
            &Function::Builtin { ref name, ref func } => (func)(&evaled_args, env),
            &Function::User { ref name, ref params, ref body, } => {
                if args.len() != params.len() {
                    return Err(format!("fn expected {} args", params.len()).into())
                }

                // Create new env with arguments, eval body with new env
                let bound_params = params
                    .iter()
                    .map(|x| x.0.to_owned())
                    .zip(args.to_owned())
                    .collect();
                let mut fn_env = Env::new(bound_params, &*env);

                match body.split_last() {
                    Some((last, rest)) => {
                        for arg in rest {
                            arg.eval(&mut fn_env)?;
                        }
                        last.eval(&mut fn_env)
                    }
                    None => Ok(Expr::Nil),
                }
            }
        }
    }
}

impl<'a> Env<'a> {
    pub fn new<E>(symbols: HashMap<String, Expr>, parent: E) -> Self
    where
        E: Into<Option<&'a Env<'a>>>,
    {
        Env {
            symbols: symbols,
            parent: parent.into(),
        }
    }

    pub fn lookup(&self, symbol: &str) -> Option<&Expr> {
        self.symbols.get(symbol).or_else(|| {
            self.parent.and_then(|p| p.lookup(symbol))
        })
    }

    pub fn define(&mut self, symbol: &str, value: Expr) {
        self.symbols.insert(symbol.to_string(), value);
    }
}

impl<'a> Default for Env<'a> {
    fn default() -> Self {
        Env {
            symbols: HashMap::new(),
            parent: None,
        }
    }
}