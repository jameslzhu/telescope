use env::Env;
use error::Result;
use forms;
use types::*;
use util::*;

impl Expr {
    pub fn eval(&self, env: Env) -> Result<Expr> {
        match *self {
            Expr::List(ref lst) => lst.eval(env),
            Expr::Sym(ref symbol) => {
                env.lookup(&symbol.0).ok_or_else(||
                    format_err!("undefined symbol: {}", symbol.0)
                )
            }
            _ => Ok(self.clone()),
        }
    }

    pub(crate) fn eval_all(exprs: &[Expr], env: Env) -> Result<Expr> {
        match exprs.split_last() {
            Some((last, rest)) => {
                for expr in rest {
                    expr.eval(env.clone())?;
                }
                last.eval(env.clone())
            }
            None => Ok(Expr::Nil),
        }
    }
}

impl List {
    pub fn eval(&self, env: Env) -> Result<Expr> {
        if let Some((first, rest)) = self.0.split_first() {
            let sym = first.sym().ok_or(format_err!("expected function call"))?;

            if forms::is_special_form(sym) {
                forms::eval(sym, rest, env)
            } else if let Ok(Expr::Func(ref func)) = first.eval(env.clone()) {
                // Eval all arguments, returning if any errors
                let evaled_args = List::eval_args(rest, env.clone())?;
                func.apply(&evaled_args, env.clone())
            } else if let Ok(Expr::Macro(ref mac)) = first.eval(env.clone()) {
                mac.apply(rest, env.clone())?.eval(env.clone())
            } else {
                Err(format_err!("could not find symbol {}", first))
            }

        } else {
            Ok(Expr::Nil)
        }
    }

    fn eval_args(args: &[Expr], env: Env) -> Result<Vec<Expr>> {
        args.iter()
            .map(|a| a.eval(env.clone()))
            .collect()
    }
}

impl Function {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn apply(&self, args: &[Expr], call_env: Env) -> Result<Expr> {
        match *self {
            Function::Builtin { ref func, .. } => (func)(args, call_env),
            Function::User { ref name, ref params, ref body, ref env } => {
                let name = if let &Some(ref n) = name { n.as_str() } else { "fn" };
                ensure_args(name, args, params.len())?;
                if args.len() != params.len() {
                    return Err(format_err!("fn expected {} args", params.len()));
                }

                // Create new env with arguments, eval body with new env
                let bound_params = params
                    .iter()
                    .map(|x| x.0.to_owned())
                    .zip(args.to_owned())
                    .collect();

                let fn_env = Env::new(bound_params, Some(env.clone()));
                Expr::eval_all(body, fn_env)
            }
        }
    }
}

impl Macro {
    pub fn apply(&self, args: &[Expr], env: Env) -> Result<Expr> {
        let name = if let Some(ref n) = self.name { n.as_str() } else { "macro" };
        ensure_args(name, args, self.params.len())?;

        // Create new env with arguments, eval body with new env
        let bound_params = self.params
            .iter()
            .map(|x| x.0.to_owned())
            .zip(args.to_owned())
            .collect();

        let fn_env = Env::new(bound_params, Some(env));

        Expr::eval_all(&self.body, fn_env)
    }
}
