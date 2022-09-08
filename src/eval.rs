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
use crate::{
    var::Var,
    parser::Token
};

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
    match ast {
        Token::Expression(un) => eval_expr(un, env),
        Token::UnaryExpression(exp, op) => {
            if op.is_none() {
                eval_expr(exp, env)
            } else {
                match op.clone().unwrap().as_str() {
                    "j" => eval_expr(exp, env).to_lat(),
                    "-" => eval_expr(exp, env).to_neg(),
                    _ => Var::impossible()
                }
            }
        }, Token::ExponentialExpression(left, right) => {
            if right.is_none() {
                eval_expr(left, env)
            } else {
                let left_val = eval_expr(left, env);
                let right_val = eval_expr(right.clone().unwrap().as_ref(), env);
                left_val + right_val
            }
        }, _ => Var::impossible()
    }
}

