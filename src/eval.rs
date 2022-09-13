/*
 * Author: Dylan Turner
 * Description: Take parser output and evaluate or adjust an environment
 */

use std::collections::HashMap;
use crate::{
    var::Var,
    parser::Token,
    builtin::BUILTIN_FUNCS,
    complex::{
        FComplex,
        IComplex
    }
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
            _ => match eval_expr(stmt, env) {
                Err(err) => format!("Error: {}", err),
                Ok(val) => val.to_string()
            }
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
        Token::Expression(un, t, f) => {
            if t.is_none() {
                return eval_expr(un, env);
            }
            
            let test = eval_expr(un, env);
            if test.is_err() {
                return test;
            }

            if test.unwrap().to_string() == "0" {
                eval_expr(f.clone().unwrap().as_ref(), env)
            } else {
                eval_expr(t.clone().unwrap().as_ref(), env)
            }
        }, Token::UnaryExpression(exp, op) => {
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
            match text.parse::<i64>() {
                Err(_) => Err(format!("Failed to parse integer {}", text)),
                Ok(val) => {
                    Ok(Var {
                        ls_data: None,
                        num_data: None,
                        int_data: Some(IComplex::new_polar(val, 0))
                    })
                }
            }
        }, Token::Number(text) => {
            match text.parse::<f64>() {
                Err(_) => Err(format!("Failed to parse number {}", text)),
                Ok(val) => {
                    Ok(Var {
                        ls_data: None,
                        num_data: Some(FComplex::new_polar(val, 0.0)),
                        int_data: None
                    })
                }
            }
        }, Token::Identifier(name) => {
            if env.vars.contains_key(name) {
                Ok(env.vars[&name.clone()].clone())
            } else {
                Err(format!("No such variable '{}'", name))
            }
        }, Token::List(items) => {
            let mut var_arr = Vec::new();
            for item in items {
                let res = eval_expr(item, env);
                if res.is_err() {
                    return res;
                }
                var_arr.push(res.unwrap());
            }
            Ok(Var {
                ls_data: Some(var_arr),
                num_data: None,
                int_data: None
            })
        }, Token::FunctionCall(name, args) => {
            // Try built in funcs first
            if name == "call" {
                // Special case w/ identifiers (don't eval)
                if args.len() < 2 {
                    Err(String::from("Two few arguments to 'call'"))
                } else {
                    // First two args should be idents
                    let lib_str;
                    if let Token::Expression(u, _, _) = args[0].clone().as_ref() {
                        if let Token::UnaryExpression(e, _) = u.clone().as_ref() {
                            if let Token::ExponentialExpression(p, _) = e.clone().as_ref() {
                                if let Token::ProductExpression(s, _, _) = p.clone().as_ref() {
                                    if let Token::SumExpression(r, _, _) = s.clone().as_ref() {
                                        if let Token::RelationalExpression(t, _, _)
                                                = r.clone().as_ref() {
                                            if let Token::Term(i) = t.clone().as_ref() {
                                                if let Token::Identifier(l) = i.clone().as_ref() {
                                                    lib_str = l;
                                                } else {
                                                    return Err(String::from(
                                                        "Expected ident for 1st arg in 'call'"
                                                    ));
                                                }  
                                            } else {
                                                return Err(String::from(
                                                    "Expected ident for 1st arg in 'call'"
                                                ));
                                            }
                                        } else {
                                            return Err(String::from(
                                                "Expected ident for 1st arg in 'call'"
                                            ));
                                        }
                                    } else {
                                        return Err(String::from(
                                            "Expected ident for 1st arg in 'call'"
                                        ));
                                    }
                                } else {
                                    return Err(String::from(
                                        "Expected ident for 1st arg in 'call'"
                                    ));
                                }
                            } else {
                                return Err(String::from("Expected ident for 1st arg in 'call'"));
                            }
                        } else {
                            return Err(String::from("Expected ident for 1st arg in 'call'"));
                        }
                    } else {
                        return Err(String::from("Expected ident for 1st argument in 'call'"));
                    }

                    let func_str;
                    if let Token::Expression(u, _, _) = args[1].clone().as_ref() {
                        if let Token::UnaryExpression(e, _) = u.clone().as_ref() {
                            if let Token::ExponentialExpression(p, _) = e.clone().as_ref() {
                                if let Token::ProductExpression(s, _, _) = p.clone().as_ref() {
                                    if let Token::SumExpression(r, _, _) = s.clone().as_ref() {
                                        if let Token::RelationalExpression(t, _, _)
                                                = r.clone().as_ref() {
                                            if let Token::Term(i) = t.clone().as_ref() {
                                                if let Token::Identifier(f) = i.clone().as_ref() {
                                                    func_str = f;
                                                } else {
                                                    return Err(String::from(
                                                        "Expected ident for 2nd arg in 'call'"
                                                    ));
                                                }  
                                            } else {
                                                return Err(String::from(
                                                    "Expected ident for 2nd arg in 'call'"
                                                ));
                                            }
                                        } else {
                                            return Err(String::from(
                                                "Expected ident for 2nd arg in 'call'"
                                            ));
                                        }
                                    } else {
                                        return Err(String::from(
                                            "Expected ident for 2nd arg in 'call'"
                                        ));
                                    }
                                } else {
                                    return Err(String::from(
                                        "Expected ident for 2nd arg in 'call'"
                                    ));
                                }
                            } else {
                                return Err(String::from("Expected ident for 2nd arg in 'call'"));
                            }
                        } else {
                            return Err(String::from("Expected ident for 2nd arg in 'call'"));
                        }
                    } else {
                        return Err(String::from("Expected ident for 2nd argument in 'call'"));
                    }

                    // TODO: Implement dynamic lib stuff
                    Err(String::from("Not implemented"))
                }
            } else  if HashMap::from(BUILTIN_FUNCS).contains_key(name.as_str()) {
                let mut eval_args = Vec::new();
                for arg in args {
                    match eval_expr(arg, env) {
                        Err(err) => return Err(err),
                        Ok(val) => eval_args.push(val)
                    }
                }
                HashMap::from(BUILTIN_FUNCS)[name.as_str()](eval_args)
            } else if env.funcs.contains_key(name) {
                let mut eval_args = Vec::new();
                for arg in args {
                    match eval_expr(arg, env) {
                        Err(err) => return Err(err),
                        Ok(val) => eval_args.push(val)
                    }
                }
                let mut f_env = env.clone();
                for i in 0..env.funcs[&name.clone()].args.len() {
                    f_env.vars.insert(
                        env.funcs[&name.clone()].args[i].clone(), eval_args[i].clone()
                    );
                }
                eval_expr(&env.funcs[&name.clone()].expr, &f_env)
            } else {
                Err(format!("No such function '{}'", name))
            }
        }, _ => Err(String::from("Impossible!"))
    }
}

