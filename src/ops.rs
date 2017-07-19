use std::collections::HashMap;
use std::ops::{Sub, Div};
use itertools::Itertools;
use error::*;
use eval::Env;
use types::{Expr, List, Vector, Function, Lambda};
use util::*;

pub fn env<'a>() -> Env<'a> {
    let table: Vec<(&str, Lambda)> = vec![
        ("not", not),
        ("or", or),
        ("and", and),
        ("print", print),
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

pub fn add(args: &[Expr], _env: &Env) -> Result<Expr> {
    // Unwrap to args
    // Check all arguments are numeric
    if args.iter().all(Expr::is_num) {
        if args.iter().any(Expr::is_flt) {
            // If any are float, promote to float and perform float addition
            Ok(Expr::from(
                args.to_vec()
                    .into_iter()
                    .map(|a| a.map_int(|x: i64| x as f64))
                    .map(|x| x.flt().unwrap())
                    .sum::<f64>(),
            ))
        } else {
            // Otherwise perform integer addition
            Ok(Expr::from(
                args.to_vec().into_iter().map(|x| x.int().unwrap()).sum::<i64>(),
            ))
        }
    } else {
        Err("#[+] expected numeric".into())
    }
}

pub fn sub(args: &[Expr], _env: &Env) -> Result<Expr> {
    // Check all arguments are numeric
    if !args.iter().all(Expr::is_num) {
        return Err("#[-] expected numeric".into());
    }

    // If any are float, promote all to float and perform float subtraction
    if args.iter().any(Expr::is_flt) {
        // If one argument, negate and return
        if args.len() == 1 {
            return Ok(Expr::from(args[0].clone().map_flt(|x| -x)));
        }
        let mut nums = args
            .iter()
            .cloned()
            .map(|a| a.map_int(|x: i64| x as f64))
            .map(|x| x.flt().unwrap());

        nums.next()
            .map(|first| nums.fold(first, Sub::sub))
            .map(Expr::from)
            .ok_or("#[-] expected 1 (negate) or 2+ (subtract) arguments".into())
    }
    // Otherwise, perform integer addition
    else {
        // If one argument, negate and return
        if args.len() == 1 {
            return Ok(Expr::from(args[0].clone().map_int(|x| -x)));
        }
        let mut nums = args.iter().map(|x| x.int().unwrap());

        nums.next()
            .map(|first| Expr::from(nums.fold(first, |acc, x| acc - x)))
            .ok_or("Expected 1 (negation) or 2+ (subtraction) arguments".into())
    }
}

pub fn mul(args: &[Expr], _env: &Env) -> Result<Expr> {
    // Check all arguments are numeric
    if args.iter().all(Expr::is_num) {
        if args.iter().any(Expr::is_flt) {
            // If any are float, promote to float and perform float multiplication
            Ok(Expr::from(
                args
                    .iter()
                    .cloned()
                    .map(|a| a.map_int(|x: i64| x as f64))
                    .map(|x| x.flt().unwrap())
                    .product::<f64>(),
            ))
        } else {
            // Otherwise perform integer multiplication
            Ok(Expr::from(
                args.into_iter().map(|x| x.int().unwrap()).product::<i64>(),
            ))
        }
    } else {
        Err("#[*] expected numeric".into())
    }
}

pub fn div(args: &[Expr], _env: &Env) -> Result<Expr> {
    ensure_min_args("[/]", args, 2)?;

    // Check all arguments are numeric
    if !args.iter().all(Expr::is_num) {
        return Err("#[/] expected numeric".into());
    }

    // If any are float, promote all to float and perform float division
    if args.iter().any(Expr::is_flt) {
        let mut nums = args
            .iter()
            .cloned()
            .map(|a| a.map_int(|x: i64| x as f64))
            .map(|x| x.flt().unwrap());

        nums.next()
            .map(|first| nums.fold(first, Div::div))
            .map(Expr::from)
            .ok_or("#[/] expected at least 2 args".into())
    }
    // Otherwise, perform integer division
    else {
        let mut nums = args.iter().map(|x| x.int().unwrap());

        nums.next()
            .ok_or("#[/] expected at least 2 args".into())
            .and_then(|first| {
                nums.map(|x| if x == 0 {
                    Err("division by zero".into())
                } else {
                    Ok(x)
                }).fold_results(first, Div::div)
            })
            .map(Expr::from)
    }
}

pub fn equal(args: &[Expr], _env: &Env) -> Result<Expr> {
    ensure_args("[=]", args, 2)?;
    Ok(Expr::from(args[0] == args[1]))
}

pub fn less(args: &[Expr], _env: &Env) -> Result<Expr> {
    ensure_args("[<]", args, 2)?;
    match (&args[0], &args[1]) {
        (&Expr::Int(ref a), &Expr::Int(ref b)) => Ok(Expr::from(a < b)),
        (&Expr::Flt(ref a), &Expr::Flt(ref b)) => Ok(Expr::from(a < b)),
        (&Expr::Str(ref a), &Expr::Str(ref b)) => Ok(Expr::from(a < b)),
        _ => Err(
            format!("comparison undefined for: {}, {}", args[0], args[1]).into(),
        ),
    }
}

pub fn less_eq(args: &[Expr], _env: &Env) -> Result<Expr> {
    ensure_args("[<=]", args, 2)?;
    match (&args[0], &args[1]) {
        (&Expr::Int(ref a), &Expr::Int(ref b)) => Ok(Expr::from(a <= b)),
        (&Expr::Flt(ref a), &Expr::Flt(ref b)) => Ok(Expr::from(a <= b)),
        (&Expr::Str(ref a), &Expr::Str(ref b)) => Ok(Expr::from(a <= b)),
        _ => Err(
            format!("comparison undefined for: {}, {}", args[0], args[1]).into(),
        ),
    }
}

pub fn greater(args: &[Expr], _env: &Env) -> Result<Expr> {
    ensure_args("[>]", args, 2)?;
    match (&args[0], &args[1]) {
        (&Expr::Int(ref a), &Expr::Int(ref b)) => Ok(Expr::from(a > b)),
        (&Expr::Flt(ref a), &Expr::Flt(ref b)) => Ok(Expr::from(a > b)),
        (&Expr::Str(ref a), &Expr::Str(ref b)) => Ok(Expr::from(a > b)),
        _ => Err(
            format!("comparison undefined for: {}, {}", args[0], args[1]).into(),
        ),
    }
}

pub fn greater_eq(args: &[Expr], _env: &Env) -> Result<Expr> {
    ensure_args("[>=]", args, 2)?;
    match (&args[0], &args[1]) {
        (&Expr::Int(ref a), &Expr::Int(ref b)) => Ok(Expr::from(a >= b)),
        (&Expr::Flt(ref a), &Expr::Flt(ref b)) => Ok(Expr::from(a >= b)),
        (&Expr::Str(ref a), &Expr::Str(ref b)) => Ok(Expr::from(a >= b)),
        _ => Err(
            format!("comparison undefined for: {}, {}", args[0], args[1]).into(),
        ),
    }
}

pub fn not(args: &[Expr], _env: &Env) -> Result<Expr> {
    ensure_args("[not]", args, 1)?;
    Ok(Expr::from(!args[0].truthiness()))
}

pub fn and(args: &[Expr], _env: &Env) -> Result<Expr> {
    args
        .into_iter()
        .map(|a| a.boolean())
        .collect::<Option<Vec<_>>>()
        .ok_or("#[and] expected boolean argument".into())
        .map(|bools| bools.iter().all(|b| *b))
        .map(Expr::from)
}

pub fn or(args: &[Expr], _env: &Env) -> Result<Expr> {
    args
        .into_iter()
        .map(|a| a.boolean())
        .collect::<Option<Vec<_>>>()
        .ok_or("#[or] expected boolean argument".into())
        .map(|bools| bools.iter().any(|b| *b))
        .map(Expr::from)
}

pub fn print(args: &[Expr], _env: &Env) -> Result<Expr> {
    ensure_args("#[print]", args, 1)?;
    println!("{}", args[0]);
    Ok(Expr::Nil)
}

pub fn first(args: &[Expr], _env: &Env) -> Result<Expr> {
    ensure_args("#[first]", args, 1)?;

    match &args[0] {
        &Expr::List(ref l) => Ok(l.0.first().cloned().unwrap_or(Expr::Nil)),
        &Expr::Vector(ref q) => Ok(q.0.first().cloned().unwrap_or(Expr::Nil)),
        _ => Err("#[first] expected list".into()),
    }
}

pub fn rest(args: &[Expr], _env: &Env) -> Result<Expr> {
    match &args[0] {
        &Expr::List(ref l) => {
            Ok(
                l.0
                    .split_first()
                    .map(|(_, rest)| Expr::List(List(rest.to_vec())))
                    .unwrap_or(Expr::Nil),
            )
        }
        &Expr::Vector(ref v) => {
            Ok(
                v.0
                    .split_first()
                    .map(|(_, rest)| Expr::Vector(Vector(rest.to_vec())))
                    .unwrap_or(Expr::Nil),
            )
        }
        _ => Err("#[rest] expected list".into()),
    }
}

// (cons item seq)
pub fn cons(args: &[Expr], _env: &Env) -> Result<Expr> {
    ensure_args("cons]", args, 2)?;

    match &args[2] {
        &Expr::List(ref l) => {
            let mut new = l.clone();
            new.0.insert(0, args[1].clone());
            Ok(Expr::List(new))
        }
        &Expr::Vector(ref v) => {
            let mut new = v.clone();
            new.0.push(args[1].clone());
            Ok(Expr::Vector(new))
        }
        _ => Err("#[cons] expected list".into()),
    }
}

// (exit)
pub fn exit(_args: &[Expr], _env: &Env) -> Result<Expr> {
    Err(ErrorKind::Exit(0).into())
}
