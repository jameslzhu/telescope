use atom::*;
use error::*;

pub fn add(v: &Vec<Value>) -> Result<Value> {
    assert!(v.len() >= 2);
    v.iter().fold(Ok(0.into()), |acc, x| acc.and_then(|a| a.add(x)))
}

pub fn sub(v: &Vec<Value>) -> Result<Value> {
    assert!(v.len() == 2);
    v[0].sub(&v[1])
}
pub fn mul(v: &Vec<Value>) -> Result<Value> {
    assert!(v.len() >= 2);
    v.iter().fold(Ok(1.into()), |acc, x| acc.and_then(|a| a.mul(x)))
}

pub fn div(v: &Vec<Value>) -> Result<Value> {
    assert!(v.len() >= 2);
    v[0].div(&v[1])
}
