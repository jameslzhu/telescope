use error::*;
use forms;
use std::collections::HashMap;
use types::*;

#[derive(Clone, Debug)]
pub struct Env<'a> {
    symbols: HashMap<String, Expr>,
    parent: Option<&'a Env<'a>>,
}

impl Expr {
    pub fn eval(&self, mut env: &mut Env) -> Result<Expr> {
        match self {
            &Expr::List(ref lst) => lst.eval(&mut env),
            &Expr::Sym(ref symbol) => {
                env.lookup(&symbol.0).cloned().ok_or(
                    format!("undefined symbol: {}", symbol.0).into()
                )
            }
            _ => Ok(self.clone()),
        }
    }
}

impl List {
    pub fn eval(&self, env: &mut Env) -> Result<Expr> {
        if let Some((first, rest)) = self.0.split_first() {
            let sym = first.sym().ok_or("expected function call")?;

            if forms::is_special_form(sym) {
                forms::eval(sym, rest, env)
            } else if let Ok(Expr::Func(ref func)) = first.eval(env) {
                func.apply(rest, env)
            } else {
                Err(format!("could not find symbol {}", first).into())
            }

        } else {
            Ok(Expr::Nil)
        }
    }
}

impl Function {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn apply<'a>(&self, args: &'a [Expr], env: &mut Env) -> Result<Expr> {
        // Eval all arguments, returning if any errors
        let evaled_args = args.iter().map(|a| a.eval(env)).collect::<Result<Vec<_>>>()?;

        // Call function on args
        match self {
            &Function::Builtin { name: _, ref func } => (func)(&evaled_args, env),
            &Function::User { name: _, ref params, ref body, } => {
                if args.len() != params.len() {
                    return Err(format!("fn expected {} args", params.len()).into());
                }

                // Create new env with arguments, eval body with new env
                let bound_params = params
                    .iter()
                    .map(|x| x.0.to_owned())
                    .zip(args.to_owned())
                    .collect();
                let mut fn_env = Env::new(bound_params, Some(env));

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
    pub fn new(symbols: HashMap<String, Expr>, parent: Option<&'a Env<'a>>) -> Self {
        Env {
            symbols: symbols,
            parent: parent,
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
