#![allow(dead_code)]

use types::{Expr, List, Vector, Symbol};
use error::*;

pub fn ensure_args(fn_name: &str, args: &[Expr], count: usize) -> Result<()> {
    ensure!(args.len() == count, "#[{}] expected {} args", fn_name, count);
    Ok(())
}

pub fn ensure_range_args(fn_name: &str, args: &[Expr], min: usize, max: usize) -> Result<()> {
    ensure!(args.len() >= min && args.len() <= max, "#[{}] expected {}-{} args", fn_name, min, max);
    Ok(())
}

pub fn ensure_min_args(fn_name: &str, args: &[Expr], count: usize) -> Result<()> {
    ensure!(args.len() >= count, "#[{}] expected at least {} args", fn_name, count);
    Ok(())
}

pub fn ensure_int(fn_name: &str, arg: &Expr) -> Result<i64> {
	arg.int().ok_or_else(|| format!("#[{}] expected integer", fn_name).into())
}

pub fn ensure_flt(fn_name: &str, arg: &Expr) -> Result<f64> {
	arg.flt().ok_or_else(|| format!("#[{}] expected float", fn_name).into())
}

pub fn ensure_sym<'a>(fn_name: &str, arg: &'a Expr) -> Result<&'a Symbol> {
	arg.sym().ok_or_else(|| format!("#[{}] expected symbol", fn_name).into())
}

pub fn ensure_list<'a>(fn_name: &str, arg: &'a Expr) -> Result<&'a List> {
	arg.list().ok_or_else(|| format!("#[{}] expected list", fn_name).into())
}

pub fn ensure_vector<'a>(fn_name: &str, arg: &'a Expr) -> Result<&'a Vector> {
	arg.vector().ok_or_else(|| format!("#[{}] expected vector", fn_name).into())
}
