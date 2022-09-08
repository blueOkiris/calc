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
            Token::Expression(sub_ast) => match eval_expr(sub_ast, env) {
                Err(err) => format!("Error: {}", err),
                Ok(val) => val.to_string()
            }, _ => String::from("Not implemented")
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
    match eval_expr(sub_expr, env) {
        Err(err) => format!("Error: {}", err),
        Ok(val) => {
            let succ = env.vars.insert(name.clone(), val);

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
    }
}

// Meat and bones - actually calculate stuff
fn eval_expr(ast: &Token, env: &Environment) -> Result<Var, String> {
    match ast {
        Token::Expression(un) => eval_expr(un, env),
        Token::UnaryExpression(exp, op) => {
            if op.is_none() {
                eval_expr(exp, env)
            } else {
                match op.clone().unwrap().as_str() {
                    "j" => {
                        let val = eval_expr(exp, env);
                        if val.is_err() {
                            val
                        } else {
                            Ok(val.unwrap().to_lat())
                        }
                    }, "-" => {
                        let val = eval_expr(exp, env);
                        if val.is_err() {
                            val
                        } else {
                            Ok(val.unwrap().to_neg())
                        }
                    }, _ => Err(String::from("Impossible!"))
                }
            }
        }, Token::ExponentialExpression(left, right) => {
            if right.is_none() {
                eval_expr(left, env)
            } else {
                let left_val = eval_expr(left, env);
                if left_val.is_err() {
                    return left_val;
                }
                let right_val = eval_expr(right.clone().unwrap().as_ref(), env);
                if right_val.is_err() {
                    return right_val;
                }
                Ok(left_val.unwrap() ^ right_val.unwrap())
            }
        }, Token::ProductExpression(left, op, right) => {
            if right.is_none() {
                eval_expr(left, env)
            } else {
                let left_val = eval_expr(left, env);
                if left_val.is_err() {
                    return left_val;
                }
                let right_val = eval_expr(right.clone().unwrap().as_ref(), env);
                if right_val.is_err() {
                    return right_val;
                }
                match op.clone().unwrap().as_str() {
                    "*" => Ok(left_val.unwrap() * right_val.unwrap()),
                    "/" => Ok(left_val.unwrap() / right_val.unwrap()),
                    _ => Err(String::from("Impossible!"))
                }
            }
        }, Token::SumExpression(left, op, right) => {
            if right.is_none() {
                eval_expr(left, env)
            } else {
                let left_val = eval_expr(left, env);
                if left_val.is_err() {
                    return left_val;
                }
                let right_val = eval_expr(right.clone().unwrap().as_ref(), env);
                if right_val.is_err() {
                    return right_val;
                }
                match op.clone().unwrap().as_str() {
                    "+" => Ok(left_val.unwrap() + right_val.unwrap()),
                    "-" => Ok(left_val.unwrap() - right_val.unwrap()),
                    _ => Err(String::from("Impossible!"))
                }
            }
        }, Token::RelationalExpression(left, op, right) => {
            if right.is_none() {
                eval_expr(left, env)
            } else {
                let left_val = eval_expr(left, env);
                if left_val.is_err() {
                    return left_val;
                }
                let right_val = eval_expr(right.clone().unwrap().as_ref(), env);
                if right_val.is_err() {
                    return right_val;
                }
                Ok(left_val.unwrap().do_cmp(right_val.unwrap(), op.clone().unwrap().as_str()))
            }
        }, Token::Term(inner) => eval_expr(inner, env),
        Token::Integer(text) => {
            match BigInt::from_str(text) {
                Err(_) => Err(format!("Failed to parse integer {}", text)),
                Ok(val) => {
                    Ok(Var {
                        ls_data: None,
                        real_num_data: None,
                        lat_num_data: None,
                        real_int_data: Some(val),
                        lat_int_data: None
                    })
                }
            }
        }, Token::Number(text) => {
            match BigDecimal::from_str(text) {
                Err(_) => Err(format!("Failed to parse number {}", text)),
                Ok(val) => {
                    Ok(Var {
                        ls_data: None,
                        real_num_data: Some(val),
                        lat_num_data: None,
                        real_int_data: None,
                        lat_int_data: None
                    })
                }
            }
        }, Token::Identifier(name) => {
            if env.vars.contains_key(name) {
                Ok(env.vars[&name.clone()].clone())
            } else {
                Err(format!("No such variable '{}'", name))
            }
        } _ => Err(String::from("Impossible!"))
    }
}

