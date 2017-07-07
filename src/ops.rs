use ast::{Atom, Expr, List, Vector, Env, Function, Lambda};
use error::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::ops::{Sub, Div};

pub fn env<'a>() -> Env<'a> {
    let table: Vec<(&str, Box<Lambda>)> = vec![
        ("not",     Box::new(not)),
        ("or",      Box::new(or)),
        ("and",     Box::new(and)),
        ("print",   Box::new(print)),
        ("+",       Box::new(add)),
        ("-",       Box::new(sub)),
        ("*",       Box::new(mul)),
        ("/",       Box::new(div)),
        ("=",       Box::new(equal)),
        ("<",       Box::new(less)),
        ("<=",      Box::new(less_eq)),
        (">",       Box::new(greater)),
        (">=",      Box::new(greater_eq)),
        ("first",   Box::new(first)),
        ("rest",    Box::new(rest)),
        ("cons",    Box::new(cons)),
        ("exit",    Box::new(exit)),
    ];

    let builtins = table.into_iter()
        .map(|(symbol, f)| {
            (String::from(symbol), Expr::from(Function::builtin(symbol, f)))
        }).collect::<HashMap<_, _>>();

    Env::new(builtins, None)
}

fn unwrap_atoms<I>(args: I) -> Result<Vec<Atom>>
where
    I: Iterator<Item = Expr>,
{
    args.map(|e| e.atom().cloned())
        .collect::<Option<Vec<_>>>()
        .ok_or("expected atom".into())
}

fn check_args(f: &str, args: &[Expr], arity: usize) -> Result<()> {
    if args.len() == arity {
        Ok(())
    } else {
        Err(format!("{} expected {} arguments", f, arity).into())
    }
}

pub fn add(args: &[Expr], _env: &Env) -> Result<Expr> {
    // Unwrap to atoms
    let atoms = unwrap_atoms(args.iter().cloned())?;

    // Check all arguments are numeric
    if atoms.iter().all(Atom::is_num) {
        if atoms.iter().any(Atom::is_flt) {
            // If any are float, promote to float and perform float addition
            Ok(Expr::from(
                atoms
                    .into_iter()
                    .map(|a| a.map_int(|x: i64| x as f64))
                    .map(|x| x.flt().unwrap())
                    .sum::<f64>(),
            ))
        } else {
            // Otherwise perform integer addition
            Ok(Expr::from(
                atoms.into_iter().map(|x| x.int().unwrap()).sum::<i64>(),
            ))
        }
    } else {
        Err("#[+] expected numeric".into())
    }
}

pub fn sub(args: &[Expr], _env: &Env) -> Result<Expr> {
    // Unwrap to atoms
    let atoms = unwrap_atoms(args.iter().cloned())?;

    // Check all arguments are numeric
    if !atoms.iter().all(Atom::is_num) {
        return Err("#[-] expected numeric".into());
    }

    // If any are float, promote all to float and perform float subtraction
    if atoms.iter().any(Atom::is_flt) {
        // If one argument, negate and return
        if atoms.len() == 1 {
            let mut atoms = atoms;
            return Ok(Expr::from(atoms.remove(0).map_flt(|x| -x)));
        }
        let mut nums = atoms
            .into_iter()
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
        if atoms.len() == 1 {
            let mut atoms = atoms;
            return Ok(Expr::from(atoms.remove(0).map_int(|x| -x)));
        }
        let mut nums = atoms.iter().map(|x| x.int().unwrap());

        nums.next()
            .map(|first| Expr::from(nums.fold(first, |acc, x| acc - x)))
            .ok_or("Expected 1 (negation) or 2+ (subtraction) arguments".into())
    }
}

pub fn mul(args: &[Expr], _env: &Env) -> Result<Expr> {
    // Unwrap to atoms
    let atoms = unwrap_atoms(args.iter().cloned())?;

    // Check all arguments are numeric
    if atoms.iter().all(Atom::is_num) {
        if atoms.iter().any(Atom::is_flt) {
            // If any are float, promote to float and perform float multiplication
            Ok(Expr::from(
                atoms
                    .into_iter()
                    .map(|a| a.map_int(|x: i64| x as f64))
                    .map(|x| x.flt().unwrap())
                    .product::<f64>(),
            ))
        } else {
            // Otherwise perform integer multiplication
            Ok(Expr::from(
                atoms.into_iter().map(|x| x.int().unwrap()).product::<i64>(),
            ))
        }
    } else {
        Err("#[*] expected numeric".into())
    }
}

pub fn div(args: &[Expr], _env: &Env) -> Result<Expr> {
    // Unwrap to atoms
    let atoms = unwrap_atoms(args.iter().cloned())?;

    // Check all arguments are numeric
    if !atoms.iter().all(Atom::is_num) {
        return Err("#[-] expected numeric".into());
    }

    // If any are float, promote all to float and perform float division
    if atoms.iter().any(Atom::is_flt) {
        let mut nums = atoms
            .into_iter()
            .map(|a| a.map_int(|x: i64| x as f64))
            .map(|x| x.flt().unwrap());

        nums.next()
            .map(|first| nums.fold(first, Div::div))
            .map(Expr::from)
            .ok_or("#[/] expected at least 2 args".into())
    }
    // Otherwise, perform integer division
    else {
        let mut nums = atoms.iter().map(|x| x.int().unwrap());

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
    check_args("#[=]", args, 2)?;
    Ok(Expr::from(args[0] == args[1]))
}

pub fn less(args: &[Expr], _env: &Env) -> Result<Expr> {
    check_args("#[<]", args, 2)?;
    match (&args[0], &args[1]) {
        (&Expr::Atom(ref a), &Expr::Atom(ref b)) => Ok(Expr::from(a < b)),
        _ => Err(
            format!("comparison undefined for: {}, {}", args[0], args[1]).into(),
        ),
    }
}

pub fn less_eq(args: &[Expr], _env: &Env) -> Result<Expr> {
    check_args("#[<=]", args, 2)?;
    match (&args[0], &args[1]) {
        (&Expr::Atom(ref a), &Expr::Atom(ref b)) => Ok(Expr::from(a <= b)),
        _ => Err(
            format!("comparison undefined for: {}, {}", args[0], args[1]).into(),
        ),
    }
}

pub fn greater(args: &[Expr], _env: &Env) -> Result<Expr> {
    check_args("#[>]", args, 2)?;
    match (&args[0], &args[1]) {
        (&Expr::Atom(ref a), &Expr::Atom(ref b)) => Ok(Expr::from(a > b)),
        _ => Err(
            format!("comparison undefined for: {}, {}", args[0], args[1]).into(),
        ),
    }
}

pub fn greater_eq(args: &[Expr], _env: &Env) -> Result<Expr> {
    check_args("#[>=]", args, 2)?;
    match (&args[0], &args[1]) {
        (&Expr::Atom(ref a), &Expr::Atom(ref b)) => Ok(Expr::from(a >= b)),
        _ => Err(
            format!("comparison undefined for: {}, {}", args[0], args[1]).into(),
        ),
    }
}

pub fn not(args: &[Expr], _env: &Env) -> Result<Expr> {
    check_args("#[not]", args, 1)?;
    match &args[0] {
        &Expr::Atom(Atom::Bool(b)) => Ok(Expr::from(!b)),
        _ => Err(format!("negation undefined for: {}", args[0]).into()),
    }
}

pub fn and(args: &[Expr], _env: &Env) -> Result<Expr> {
    // Unwrap to atoms
    let atoms = unwrap_atoms(args.iter().cloned())?;

    atoms
        .into_iter()
        .map(|a| a.boolean())
        .collect::<Option<Vec<_>>>()
        .ok_or("#[and] expected boolean argument".into())
        .map(|bools| bools.iter().all(|b| *b))
        .map(Expr::from)
}

pub fn or(args: &[Expr], _env: &Env) -> Result<Expr> {
    // Unwrap to atoms
    let atoms = unwrap_atoms(args.iter().cloned())?;

    atoms
        .into_iter()
        .map(|a| a.boolean())
        .collect::<Option<Vec<_>>>()
        .ok_or("#[or] expected boolean argument".into())
        .map(|bools| bools.iter().any(|b| *b))
        .map(Expr::from)
}

pub fn print(args: &[Expr], _env: &Env) -> Result<Expr> {
    check_args("#[print]", args, 1)?;
    println!("{}", args[0]);
    Ok(Expr::Nil)
}

pub fn first(args: &[Expr], _env: &Env) -> Result<Expr> {
    check_args("#[first]", args, 1)?;

    match &args[0] {
        &Expr::List(ref l) => Ok(l.0.first().cloned().unwrap_or(Expr::Nil)),
        &Expr::Vector(ref q) => Ok(q.0.first().cloned().unwrap_or(Expr::Nil)),
        _ => Err("#[first] expected list".into()),
    }
}

pub fn rest(args: &[Expr], _env: &Env) -> Result<Expr> {
    match &args[0] {
        &Expr::List(ref l) => {
            Ok(l.0.split_first()
                .map(|(_, rest)| {
                    Expr::List(List(rest.to_vec()))
                }).unwrap_or(Expr::Nil))
        },
        &Expr::Vector(ref v) => {
            Ok(v.0.split_first()
                .map(|(_, rest)| {
                    Expr::Vector(Vector(rest.to_vec()))
                }).unwrap_or(Expr::Nil))
        },
        _ => Err("#[rest] expected list".into()),
    }
}

// (cons item seq)
pub fn cons(args: &[Expr], _env: &Env) -> Result<Expr> {
    check_args("#[cons]", args, 2)?;

    match &args[2] {
        &Expr::List(ref l) => {
            let mut new = l.clone();
            new.0.insert(0, args[1].clone());
            Ok(Expr::List(new))
        },
        &Expr::Vector(ref v) => {
            let mut new = v.clone();
            new.0.push(args[1].clone());
            Ok(Expr::Vector(new))
        },
        _ => Err("#[cons] expected list".into()),
    }
}

// (exit)
pub fn exit(_args: &[Expr], _env: &Env) -> Result<Expr> {
    Err(ErrorKind::Exit(0).into())
}