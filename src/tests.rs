/*
 * Author: Dylan Turner
 * Description: Any unit testing done for the project
 */

#[cfg(test)]

use crate::parser::retrieve_integer;

#[test]
pub fn parse_ints() {
    assert!(retrieve_integer("1").is_none());
    assert!(retrieve_integer("1_").is_some());
    assert!(retrieve_integer("1.2_").is_none());
    assert!(retrieve_integer("1_000_000_").is_some());
}

