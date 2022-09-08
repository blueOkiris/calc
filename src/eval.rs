/*
 * Author: Dylan Turner
 * Description: Take parser output and evaluate or adjust an environment
 */

use bigdecimal::BigDecimal;
use num::BigInt;
use std::{
    collections::HashMap,
    str::FromStr
};
use crate::parser::Token;

// Can be list, complex float, or complex int
#[derive(Clone, Debug)]
pub struct Var {
    pub ls_data: Option<Vec<Var>>,
    pub real_num_data: Option<BigDecimal>,
    pub lat_num_data: Option<BigDecimal>,
    pub real_int_data: Option<BigInt>,
    pub lat_int_data: Option<BigInt>
}

pub struct Environment {
    pub vars: HashMap<String, Var>,
    pub funcs: HashMap<String, Token>
}

impl Environment {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            funcs: HashMap::new()
        }
    }
}

pub fn eval(ast: &Token, env: &mut Environment) -> String {
    String::from("Not implemented")
}

