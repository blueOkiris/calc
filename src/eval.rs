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

// Mainly the base functions 
impl Var {
    pub fn to_string(&self) -> String {
        let mut repr = String::new();
        if self.ls_data.is_none() {
            if self.real_num_data.is_some() {
                repr.push_str(self.real_num_data.clone().unwrap().to_string().as_str());
            } else if self.real_int_data.is_some() {
                repr.push_str(self.real_int_data.clone().unwrap().to_string().as_str());
            }
            if self.lat_num_data.is_some() {
                if repr.len() > 0 {
                    repr.push('+');
                }
                repr.push('j');
                repr.push_str(self.lat_num_data.clone().unwrap().to_string().as_str());
            } else {
                if repr.len() > 0 {
                    repr.push('+');
                }
                repr.push('j');
                repr.push_str(self.lat_int_data.clone().unwrap().to_string().as_str());
            }
        } else {
            repr.push_str("[ ");
            for var in self.ls_data.clone().unwrap() {
                repr.push_str(var.to_string().as_str());
                repr.push(' ');
            }
            repr.push(']');
        }
        if repr.len() < 1 {
            repr = String::from("IMPOSSIBLE DATA ACHIEVED")
        }
        repr
    }

    pub fn impossible() -> Self {
        Self {
            ls_data: None,
            real_num_data: None,
            lat_num_data: None,
            real_int_data: None,
            lat_int_data: None
        }
    }

    pub fn to_lat(&self) -> Self {
        let mut new_self = self.clone();

        if new_self.real_num_data.is_some() {
            new_self.lat_num_data = new_self.real_num_data;
            new_self.real_num_data = None;
        } else if new_self.real_int_data.is_some() {
            new_self.lat_int_data = new_self.real_int_data;
            new_self.real_int_data = None;
        } else if new_self.ls_data.is_some() {
            let mut new_ls = Vec::new();
            let ls = new_self.ls_data.unwrap();
            for var in ls {
                new_ls.push(var.to_lat());
            }
            new_self.ls_data = Some(new_ls);
        }
        
        new_self
    }

    pub fn to_neg(&self) -> Self {
        let mut new_self = self.clone();

        if new_self.real_num_data.is_some() {
            new_self.real_num_data = Some(-new_self.real_num_data.unwrap());
        } else if new_self.real_int_data.is_some() {
            new_self.real_int_data = Some(-new_self.real_int_data.unwrap());
        } else if new_self.lat_num_data.is_some() {
            new_self.lat_num_data = Some(-new_self.lat_num_data.unwrap());
        } else if new_self.lat_int_data.is_some() {
            new_self.lat_int_data = Some(-new_self.lat_int_data.unwrap());
        } else if new_self.ls_data.is_some() {
            let mut new_ls = Vec::new();
            let ls = new_self.ls_data.unwrap();
            for var in ls {
                new_ls.push(var.to_neg());
            }
            new_self.ls_data = Some(new_ls);
        }
        
        new_self
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
        }, _ => Var::impossible()
    }
}

