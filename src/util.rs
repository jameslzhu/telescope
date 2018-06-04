#![allow(dead_code)]

use types::{Expr, List, Vector, Symbol};
// use error::*;
use error::Error;
// use failure::Error;

pub fn ensure_args(fn_name: &str, args: &[Expr], count: usize) -> Result<(), Error> {
    if args.len() == count {
        Ok(())
    } else {
        Err(Error::Msg(format!("#[{}] expected {} args", fn_name, count)))
    }
}

pub fn ensure_range_args(fn_name: &str, args: &[Expr], min: usize, max: usize) -> Result<(), Error> {
    if args.len() >= min && args.len() <= max {
        Ok(())
    } else {
        Err(Error::Msg(format!("#[{}] expected {}-{} args", fn_name, min, max)))
    }
}

pub fn ensure_min_args(fn_name: &str, args: &[Expr], count: usize) -> Result<(), Error> {
    if args.len() >= count {
        Ok(())
    } else {
        Err(Error::Msg(format!("#[{}] expected at least {} args", fn_name, count)))
    }
}

pub fn ensure_int(fn_name: &str, arg: &Expr) -> Result<i64, Error> {
    arg.int().ok_or_else(|| Error::Msg(format!("#[{}] expected integer", fn_name)))
}

pub fn ensure_flt(fn_name: &str, arg: &Expr) -> Result<f64, Error> {
    arg.flt().ok_or_else(|| Error::Msg(format!("#[{}] expected float", fn_name)))
}

pub fn ensure_sym<'a>(fn_name: &str, arg: &'a Expr) -> Result<&'a Symbol, Error> {
    arg.sym().ok_or_else(|| Error::Msg(format!("#[{}] expected symbol", fn_name)))
}

pub fn ensure_list<'a>(fn_name: &str, arg: &'a Expr) -> Result<&'a List, Error> {
    arg.list().ok_or_else(|| Error::Msg(format!("#[{}] expected list", fn_name)))
}

pub fn ensure_vector<'a>(fn_name: &str, arg: &'a Expr) -> Result<&'a Vector, Error> {
    arg.vector().ok_or_else(|| Error::Msg(format!("#[{}] expected vector", fn_name)))
}
