use atom::*;
use error::*;

pub fn add(v: &[Value]) -> Result<Value> {
    assert!(v.len() >= 2);
    v.iter().fold(Ok(0.into()), |acc, x| acc.and_then(|a| a.add(x)))
}

pub fn sub(v: &[Value]) -> Result<Value> {
    assert!(v.len() == 1 || v.len() == 2);
    if v.len() == 1 {
        v[0].neg()
    } else {
        v[0].sub(&v[1])
    }
}

pub fn mul(v: &[Value]) -> Result<Value> {
    assert!(v.len() >= 2);
    v.iter().fold(Ok(1.into()), |acc, x| acc.and_then(|a| a.mul(x)))
}

pub fn div(v: &[Value]) -> Result<Value> {
    assert!(v.len() >= 2);
    v[0].div(&v[1])
}

pub fn modulus(v: &[Value]) -> Result<Value> {
    assert!(v.len() == 2);
    v[0].modulus(&v[1])
}
