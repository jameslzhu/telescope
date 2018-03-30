#![allow(dead_code)]

use types::{Expr, List, Vector, Symbol};
// use error::*;
use failure::Error;

pub fn ensure_args(fn_name: &str, args: &[Expr], count: usize) -> Result<(), Error> {
    ensure!(args.len() == count, "#[{}] expected {} args", fn_name, count);
    Ok(())
}

pub fn ensure_range_args(fn_name: &str, args: &[Expr], min: usize, max: usize) -> Result<(), Error> {
    ensure!(args.len() >= min && args.len() <= max, "#[{}] expected {}-{} args", fn_name, min, max);
    Ok(())
}

pub fn ensure_min_args(fn_name: &str, args: &[Expr], count: usize) -> Result<(), Error> {
    ensure!(args.len() >= count, "#[{}] expected at least {} args", fn_name, count);
    Ok(())
}

pub fn ensure_int(fn_name: &str, arg: &Expr) -> Result<i64, Error> {
	arg.int().ok_or_else(|| format_err!("#[{}] expected integer", fn_name))
}

pub fn ensure_flt(fn_name: &str, arg: &Expr) -> Result<f64, Error> {
	arg.flt().ok_or_else(|| format_err!("#[{}] expected float", fn_name))
}

pub fn ensure_sym<'a>(fn_name: &str, arg: &'a Expr) -> Result<&'a Symbol, Error> {
	arg.sym().ok_or_else(|| format_err!("#[{}] expected symbol", fn_name))
}

pub fn ensure_list<'a>(fn_name: &str, arg: &'a Expr) -> Result<&'a List, Error> {
	arg.list().ok_or_else(|| format_err!("#[{}] expected list", fn_name))
}

pub fn ensure_vector<'a>(fn_name: &str, arg: &'a Expr) -> Result<&'a Vector, Error> {
	arg.vector().ok_or_else(|| format_err!("#[{}] expected vector", fn_name))
}
