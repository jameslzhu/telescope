use types::Expr;
use error::*;

pub fn ensure_args(fn_name: &str, args: &[Expr], count: usize) -> Result<()> {
    ensure!(args.len() == count, "#[{}] expected {} args", fn_name, count);
    Ok(())
}

pub fn ensure_min_args(fn_name: &str, args: &[Expr], count: usize) -> Result<()> {
    ensure!(args.len() >= count, "#[{}] expected at least {} args", fn_name, count);
    Ok(())
}