#![allow(unused_variables)]

use ast::{Atom, Expr};
use error::*;

fn eval_args(args: &[Expr]) -> Result<Vec<Expr>> {
    args.iter()
        .map(Expr::eval)
        .collect::<Result<_>>()
        .chain_err(|| "argument eval failed")
}

fn unwrap_atoms<I>(args: I) -> Result<Vec<Atom>>
    where I: Iterator<Item = Expr>
{
    args.map(Expr::atom)
        .collect::<Option<Vec<_>>>()
        .ok_or("expected atom".into())
}

pub fn add(args: &[Expr]) -> Result<Expr> {
    // Evaluate all arguments
    let evaled_args = eval_args(args)?;
    
    // Unwrap to atoms
    let atoms = unwrap_atoms(evaled_args.into_iter())?;
    
    // Check all arguments are numeric
    if atoms.iter().all(Atom::is_num) {
        if atoms.iter().any(Atom::is_flt) {
            // If any are float, promote to float and perform float addition
            Ok(Expr::from(atoms.into_iter()
                .map(|a: Atom| a.map_int(|x: i64| Atom::Flt(x as f64)))
                .map(|x| x.flt().unwrap())
                .sum::<f64>()))
        } else {
            // Otherwise perform integer addition
            Ok(Expr::from(atoms.into_iter()
                .map(|x| x.int().unwrap())
                .sum::<i64>()))
        }
    } else {
        Err("could not add non-numeric atom".into())
    }
}

pub fn sub(args: &[Expr]) -> Result<Expr> {
    // Evaluate all arguments
    let evaled_args = eval_args(args)?;
    
    // Unwrap to atoms
    let atoms = unwrap_atoms(evaled_args.into_iter())?;

    // Check all arguments are numeric
    if atoms.iter().all(Atom::is_num) {
        // If any are float, promote all to float and perform float addition
        if atoms.iter().any(Atom::is_flt) {
            // If one argument, negate and return
            if atoms.len() == 1 {
                let mut atoms = atoms;
                return Ok(Expr::from(atoms.remove(0).map_flt(|x| Atom::from(-x))))
            }
            let mut nums = atoms.into_iter()
                .map(|a: Atom| a.map_int(|x: i64| Atom::Flt(x as f64)))
                .map(|x| x.flt().unwrap());
                
            nums.next()
                .map(|first| {
                    Expr::from(nums.fold(first, |acc, x| acc - x))
                }).ok_or("incorrect number of arguments".into())
        }
        // Otherwise, perform integer addition
        else {
            // If one argument, negate and return
            if atoms.len() == 1 {
                let mut atoms = atoms;
                return Ok(Expr::from(atoms.remove(0).map_int(|x| Atom::from(-x))))
            }
            let mut nums = atoms.iter()
                .map(|x| x.int().unwrap());
            
            nums.next()
                .map(|first| {
                    Expr::from(nums.fold(first, |acc, x| acc - x))
                }).ok_or("incorrect number of arguments".into())
        }
    } else {
        Err("could not subtract non-numeric atom".into())
    }
}

pub fn mul(args: &[Expr]) -> Result<Expr> {
    // Evaluate all arguments
    let evaled_args = eval_args(args)?;
    
    // Unwrap to atoms
    let atoms = unwrap_atoms(evaled_args.into_iter())?;
    
    // Check all arguments are numeric
    if atoms.iter().all(Atom::is_num) {
        if atoms.iter().any(Atom::is_flt) {
            // If any are float, promote to float and perform float addition
            Ok(Expr::from(atoms.into_iter()
                .map(|a: Atom| a.map_int(|x: i64| Atom::Flt(x as f64)))
                .map(|x| x.flt().unwrap())
                .product::<f64>()))
        } else {
            // Otherwise perform integer addition
            Ok(Expr::from(atoms.into_iter()
                .map(|x| x.int().unwrap())
                .product::<i64>()))
        }
    } else {
        Err("could not multiply non-numeric atom".into())
    }
}

pub fn div(args: &[Expr]) -> Result<Expr> {
    // Evaluate all arguments
    let evaled_args = eval_args(args)?;
    
    // Unwrap to atoms
    let atoms = unwrap_atoms(evaled_args.into_iter())?;

    // Check all arguments are numeric
    if atoms.iter().all(Atom::is_num) {
        // If any are float, promote all to float and perform float addition
        if atoms.iter().any(Atom::is_flt) {
            let mut nums = atoms.into_iter()
                .map(|a: Atom| a.map_int(|x: i64| Atom::Flt(x as f64)))
                .map(|x| x.flt().unwrap());
                
            nums.next()
                .map(|first| {
                    Expr::from(nums.fold(first, |acc, x| acc / x))
                }).ok_or("incorrect number of arguments".into())
        }
        // Otherwise, perform integer addition
        else {
            let mut nums = atoms.iter()
                .map(|x| x.int().unwrap());
            
            nums.next()
                .ok_or("incorrect number of arguments".into())
                .and_then(|mut first| {
                    for num in nums {
                        if num == 0 {
                            return Err("division by zero".into());
                        }
                        first /= num;
                    }
                    Ok(Expr::from(first))
                })
        }
    } else {
        Err("could not subtract non-numeric atom".into())
    }
}

pub fn equal(args: &[Expr]) -> Result<Expr> {
    unimplemented!()
}

pub fn less(args: &[Expr]) -> Result<Expr> {
    unimplemented!()
}

pub fn less_eq(args: &[Expr]) -> Result<Expr> {
    unimplemented!()
}

pub fn greater(args: &[Expr]) -> Result<Expr> {
    unimplemented!()
}

pub fn greater_eq(args: &[Expr]) -> Result<Expr> {
    unimplemented!()
}

pub fn not(args: &[Expr]) -> Result<Expr> {
    unimplemented!()
}

pub fn and(args: &[Expr]) -> Result<Expr> {
    unimplemented!()
}

pub fn or(args: &[Expr]) -> Result<Expr> {
    unimplemented!()
}

pub fn print(args: &[Expr]) -> Result<Expr> {
    unimplemented!()
}
