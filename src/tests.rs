/*
 * Author: Dylan Turner
 * Description: Any unit testing done for the project
 */

#[cfg(test)]

use crate::parser::{
    retrieve_integer,
    retrieve_number,
    retrieve_ident
};

#[test]
pub fn parse_ints() {
    assert!(retrieve_integer("1").is_none());
    assert!(retrieve_integer("1_").is_some());
    assert!(retrieve_integer("1.2_").is_none());
    assert!(retrieve_integer("1_000_000_").is_some());
    assert!(retrieve_integer("-10_").is_some());
}

#[test]
pub fn parse_floats() {
    assert!(retrieve_number("1").is_some());
    assert!(retrieve_number("1.2_").is_some());
    assert!(retrieve_number("-1.2").is_some());
    assert!(retrieve_number(".9E-7").is_some());
    assert!(retrieve_number("6.022E24").is_some());
}

#[test]
pub fn parse_names() {
    assert!(retrieve_ident("").is_none());
    assert!(retrieve_ident("aA_9_d_.a").is_some());
    assert!(retrieve_ident("9a").is_none());
    assert!(retrieve_ident("a").is_some());
}

