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

impl Var {
    pub fn to_string(&self) -> String {
        String::from("Not implemented")
    }
}

#[derive(Clone, Debug)]
pub struct Func {
    pub args: Vec<String>,
    pub expr: Token
}

#[derive(Clone, Debug)]
pub struct Environment {
    pub vars: HashMap<String, Var>,
    pub funcs: HashMap<String, Func>
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
    if let Token::Statement(stmt) = ast {
        match stmt.as_ref() {
            Token::FunctionDefinition(name, args, sub_expr) => eval_func_def(
                name, args, sub_expr, env
            ), Token::Assignment(name, sub_expr) => eval_asgn(name, sub_expr, env),
            Token::Expression(sub_ast) => eval_expr(sub_ast, env).to_string(),
            _ => String::from("Not implemented")
        }
    } else {
        String::from("Not implemented")
    }
}

// Add a function for later
fn eval_func_def(
        name: &String, args: &Vec<String>, expr: &Token, env: &mut Environment) -> String {
    let succ = env.funcs.insert(
        name.clone(),
        Func {
            args: args.clone(),
            expr: expr.clone()
        }
    );

    // Newly added
    if succ.is_none() {
        format!("Added function '{}' with args {:?} to environment.", name, args)
    } else {
        // Updated
        format!("Updated function '{}'", name)
    }
}

// Add/set an identifier
fn eval_asgn(name: &String, sub_expr: &Token, env: &mut Environment) -> String {
    let succ = env.vars.insert(name.clone(), eval_expr(sub_expr, env));

    // Newly added
    if succ.is_none() {
        format!(
            "Added var '{}' with value {} to environment.",
            name, env.vars[&name.clone()].to_string()
        )
    } else {
        // Updated
        format!("Updated var '{}' to {}", name, env.vars[&name.clone()].to_string())
    }
}

// Meat and bones - actually calculate stuff
fn eval_expr(ast: &Token, env: &Environment) -> Var {
    Var {
        ls_data: None,
        real_int_data: None,
        lat_int_data: None,
        real_num_data: None,
        lat_num_data: None
    }
}

