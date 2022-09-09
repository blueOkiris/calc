/*
 * Author: Dylan Turner
 * Description: All the standard builtin functions. Feel free to contribute and add!
 */

use crate::var::Var;

pub const BUILTIN_FUNCS: [(&str, &fn(Vec<Var>)->Result<Var, String>); 22] = [
    ("sin", &SIN),
    ("cos", &COS),
    ("tan", &TAN),
    ("asin", &ASIN),
    ("acos", &ACOS),
    ("atan", &ATAN),
    ("d2r", &D2R),
    ("r2d", &R2D),
    ("log", &LOG),
    ("ln", &LN),
    ("e", &E),
    ("pi", &PI),
    ("mod", &MOD),
    ("floor", &FLOOR),
    ("ceil", &CEIL),
    ("abs", &ABS),
    ("idx", &IDX),
    ("len", &LEN),
    ("app", &APP),
    ("del", &DEL),
    ("sign", &SIGN),
    ("comp", &COMP)
];

// TODO: Implement the builtin functions
pub const SIN: fn(Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const COS: fn(Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const TAN: fn(Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const ASIN: fn(Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const ACOS: fn(Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const ATAN: fn(Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const D2R: fn(Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const R2D: fn(Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const LOG: fn(Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const LN: fn(Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const E: fn(Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const PI: fn(Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const MOD: fn(Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const FLOOR: fn(Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const CEIL: fn(Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const ABS: fn(Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const IDX: fn(Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const LEN: fn(Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const APP: fn(Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const DEL: fn(Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const SIGN: fn(Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const COMP: fn(Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));

