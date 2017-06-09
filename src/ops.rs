#![allow(unused_variables)]

use ast::{Atom, Expr};
use error::*;

fn eval_args(args: &[Expr]) -> Result<Vec<Expr>> {
    args.iter().map(Expr::eval).collect()
}

pub fn add(args: &[Expr]) -> Result<Expr> {
    // Evaluate all arguments
    match eval_args(args) {
        Ok(ok_args) => {
            // Convert all args to atom
            let atoms: Option<Vec<&Atom>> = ok_args.iter().map(Expr::atom).collect();
            match atoms {
                Some(a) => {
                    if a.iter().all(|x| x.is_flt() || x.is_int()) {
                        // If any are float, promote to float and perform float addition
                        if a.iter().any(|x| x.is_flt()) {
                            Ok(Expr::from(a.iter().map(|x| {
                                if let Some(float) = x.flt() { float } else { x.int().unwrap() as f64 }
                            }).sum::<f64>()))
                        } else {
                            Ok(Expr::from(a.iter()
                                .map(|x| x.int())
                                .map(Option::unwrap)
                                .sum::<i64>()))
                        }
                    } else {
                        Err("could not add non-numeric atom".into())
                    }

                },
                None => Err("could not add non-atom".into()),
            }
        },
        Err(err) => Err(format!("could not eval: {}", err).into()),
    }
}

pub fn sub(args: &[Expr]) -> Result<Expr> {
    unimplemented!()
}

pub fn mul(args: &[Expr]) -> Result<Expr> {
    unimplemented!()
}

pub fn div(args: &[Expr]) -> Result<Expr> {
    unimplemented!()
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
