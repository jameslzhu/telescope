extern crate nom;

use std::str;

use nom::IResult;
use nom::{digit, multispace};

named!(pub numeric<i64>,
    map_res!(
        map_res!(
            delimited!(opt!(multispace), digit, opt!(multispace)),
            str::from_utf8
        ),
        str::FromStr::from_str
    )
);

#[test]
fn test_numeric() {
    let input = "  01234 ";
    
    if let IResult::Done(_, result) = numeric(input.as_bytes()) {
        assert!(result == 1234);
    }
}