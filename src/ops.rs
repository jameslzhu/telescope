use std::collections::HashMap;
use std::ops::{Sub, Div};
use itertools::Itertools;
use error::*;
use env::Env;
use types::{Expr, List, Vector, Function, Lambda};
use util::*;

pub fn env() -> Env {
    let table: Vec<(&str, Lambda)> = vec![
        ("not", not),
        ("+", add),
        ("-", sub),
        ("*", mul),
        ("/", div),
        ("=", equal),
        ("<", less),
        ("<=", less_eq),
        (">", greater),
        (">=", greater_eq),
        ("first", first),
        ("rest", rest),
        ("cons", cons),
        ("list", list),
        ("print", print),
        ("debug", debug),
        ("eval", eval),
        ("exit", exit),
    ];

    let builtins = table
        .into_iter()
        .map(|(symbol, f)| {
            (
                String::from(symbol),
                Expr::from(Function::builtin(symbol, f)),
            )
        })
        .collect::<HashMap<_, _>>();

    Env::new(builtins, None)
}

fn numeric_op<F, G>(name: &str, args: &[Expr], fn_int: F, fn_flt: G) -> Result<Expr>
where
    F: Fn(&[i64]) -> Result<i64>,
    G: Fn(&[f64]) -> Result<f64>,
{
    // Check all arguments are numeric
    if args.iter().all(Expr::is_num) {
        if args.iter().any(Expr::is_flt) {
            // If any are float, promote to float
            let floats = args.iter()
                .map(|x| match *x {
                    Expr::Int(y) => y as f64,
                    Expr::Flt(y) => y,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>();
            fn_flt(&floats).map(Expr::from)
        } else {
            // Otherwise perform integer operation
            let ints = args.iter()
                .map(|x| match *x {
                    Expr::Int(y) => y,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>();
            fn_int(&ints).map(Expr::from)
        }
    } else {
        Err(Error::Msg(format!("#[{}] expected numeric", name)))
    }
}

fn add(args: &[Expr], _env: Env) -> Result<Expr> {
    numeric_op("+", args,
        |ints| Ok(ints.iter().sum::<i64>()),
        |floats| Ok(floats.iter().sum::<f64>())
    )
}

fn sub(args: &[Expr], _env: Env) -> Result<Expr> {
    ensure_min_args("-", args, 1)?;

    // If one argument, negate and return
    if args.len() == 1 {
        return match args[0] {
            Expr::Int(x) => Ok(Expr::from(-x)),
            Expr::Flt(x) => Ok(Expr::from(-x)),
            _ => Err(Error::Msg(format!("invalid type")))
        }
    }

    numeric_op("-", args,
        |ints| Ok(ints[1..].iter().fold(ints[0], Sub::sub)),
        |floats| Ok(floats[1..].iter().fold(floats[0], Sub::sub))
    )
}

fn mul(args: &[Expr], _env: Env) -> Result<Expr> {
    numeric_op("*", args,
        |ints| Ok(ints.iter().product::<i64>()),
        |floats| Ok(floats.iter().product::<f64>())
    )
}

fn div(args: &[Expr], _env: Env) -> Result<Expr> {
    ensure_min_args("/", args, 1)?;

    // If one argument, invert and return
    if args.len() == 1 {
        return match args[0] {
            Expr::Int(x) => Ok(Expr::from((x as f64).recip())),
            Expr::Flt(x) => Ok(Expr::from(x.recip())),
            _ => Err(Error::Msg(format!("invalid type")))
        }
    }

    let int_div = |ints: &[i64]| {
        ints[1..].iter()
            .map(|&x| if x == 0i64 { Err(Error::Msg(format!("division by zero"))) } else { Ok(x) })
            .fold_results(ints[0], Div::div)
    };

    numeric_op("/", args,
        int_div,
        |floats| Ok(floats[1..].iter().fold(floats[0], Div::div))
    )
}

fn equal(args: &[Expr], _env: Env) -> Result<Expr> {
    ensure_args("=", args, 2)?;
    Ok(Expr::from(args[0] == args[1]))
}

fn less(args: &[Expr], _env: Env) -> Result<Expr> {
    ensure_args("<", args, 2)?;
    match (&args[0], &args[1]) {
        (&Expr::Int(ref a), &Expr::Int(ref b)) => Ok(Expr::from(a < b)),
        (&Expr::Flt(ref a), &Expr::Flt(ref b)) => Ok(Expr::from(a < b)),
        (&Expr::Str(ref a), &Expr::Str(ref b)) => Ok(Expr::from(a < b)),
        _ => Err(
            Error::Msg(format!("comparison undefined for: {}, {}", args[0], args[1])),
        ),
    }
}

fn less_eq(args: &[Expr], _env: Env) -> Result<Expr> {
    ensure_args("<=", args, 2)?;
    match (&args[0], &args[1]) {
        (&Expr::Int(ref a), &Expr::Int(ref b)) => Ok(Expr::from(a <= b)),
        (&Expr::Flt(ref a), &Expr::Flt(ref b)) => Ok(Expr::from(a <= b)),
        (&Expr::Str(ref a), &Expr::Str(ref b)) => Ok(Expr::from(a <= b)),
        _ => Err(
            Error::Msg(format!("comparison undefined for: {}, {}", args[0], args[1])),
        ),
    }
}

fn greater(args: &[Expr], _env: Env) -> Result<Expr> {
    ensure_args(">", args, 2)?;
    match (&args[0], &args[1]) {
        (&Expr::Int(ref a), &Expr::Int(ref b)) => Ok(Expr::from(a > b)),
        (&Expr::Flt(ref a), &Expr::Flt(ref b)) => Ok(Expr::from(a > b)),
        (&Expr::Str(ref a), &Expr::Str(ref b)) => Ok(Expr::from(a > b)),
        _ => Err(
            Error::Msg(format!("comparison undefined for: {}, {}", args[0], args[1])),
        ),
    }
}

fn greater_eq(args: &[Expr], _env: Env) -> Result<Expr> {
    ensure_args(">=", args, 2)?;
    match (&args[0], &args[1]) {
        (&Expr::Int(ref a), &Expr::Int(ref b)) => Ok(Expr::from(a >= b)),
        (&Expr::Flt(ref a), &Expr::Flt(ref b)) => Ok(Expr::from(a >= b)),
        (&Expr::Str(ref a), &Expr::Str(ref b)) => Ok(Expr::from(a >= b)),
        _ => Err(
            Error::Msg(format!("comparison undefined for: {}, {}", args[0], args[1])),
        ),
    }
}

// (not expr)
fn not(args: &[Expr], _env: Env) -> Result<Expr> {
    ensure_args("[not]", args, 1)?;
    Ok(Expr::from(!args[0].truthiness()))
}

// (print expr)
// TODO: lift one-argument restriction
// TODO: create print, println versions
fn print(args: &[Expr], _env: Env) -> Result<Expr> {
    ensure_args("print", args, 1)?;
    println!("{}", args[0]);
    Ok(Expr::Nil)
}

// (debug expr)
// TODO: lift one-argument restriction
fn debug(args: &[Expr], _env: Env) -> Result<Expr> {
    ensure_args("debug", args, 1)?;
    println!("{:?}", args[0]);
    Ok(Expr::Nil)
}

// (first seq)
fn first(args: &[Expr], _env: Env) -> Result<Expr> {
    ensure_args("first", args, 1)?;
    match args[0] {
        Expr::List(ref l) => Ok(l.0.first().cloned().unwrap_or(Expr::Nil)),
        Expr::Vector(ref q) => Ok(q.0.first().cloned().unwrap_or(Expr::Nil)),
        _ => Err(Error::Msg(format!("#[first] expected list"))),
    }
}

// (rest seq)
fn rest(args: &[Expr], _env: Env) -> Result<Expr> {
    ensure_args("first", args, 1)?;
    match args[0] {
        Expr::List(ref l) => {
            Ok(
                l.0
                    .split_first()
                    .map(|(_, rest)| Expr::List(List(rest.to_vec())))
                    .unwrap_or(Expr::Nil),
            )
        }
        Expr::Vector(ref v) => {
            Ok(
                v.0
                    .split_first()
                    .map(|(_, rest)| Expr::Vector(Vector(rest.to_vec())))
                    .unwrap_or(Expr::Nil),
            )
        }
        _ => Err(Error::Msg(format!("#[rest] expected list"))),
    }
}

// (cons item seq)
fn cons(args: &[Expr], _env: Env) -> Result<Expr> {
    ensure_args("cons", args, 2)?;

    match args[2] {
        Expr::List(ref l) => {
            let mut new = l.clone();
            new.0.insert(0, args[1].clone());
            Ok(Expr::List(new))
        }
        Expr::Vector(ref v) => {
            let mut new = v.clone();
            new.0.push(args[1].clone());
            Ok(Expr::Vector(new))
        }
        _ => Err(Error::Msg(format!("#[cons] expected list"))),
    }
}

// (list items*)
fn list(args: &[Expr], _env: Env) -> Result<Expr> {
    if args.is_empty() {
        Ok(Expr::Nil)
    } else {
        Ok(Expr::List(List(args.to_vec())))
    }
}

// (eval form)
fn eval(args: &[Expr], env: Env) -> Result<Expr> {
    ensure_args("eval", args, 1)?;
    args[0].eval(env)
}

// (exit)
fn exit(_args: &[Expr], _env: Env) -> Result<Expr> {
    Err(Error::Exit(0).into())
}
