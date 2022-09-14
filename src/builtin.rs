/*
 * Author: Dylan Turner
 * Description: All the standard builtin functions. Feel free to contribute and add!
 */

use std::f64::consts;
use crate::{
    var::Var,
    complex::{
        FComplex,
        IComplex
    }
};

pub const BUILTIN_FUNCS: [(&str, &fn(&Vec<Var>)->Result<Var, String>); 22] = [
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

pub const SIN: fn(&Vec<Var>)->Result<Var, String> = |vars| {
    if vars.len() != 1 {
        Err(String::from("Expected one argument for sin."))
    } else if vars[0].ls_data.is_some() {
        let mut results = Vec::new();
        for var in vars[0].clone().ls_data.unwrap() {
            match SIN(&vec![ var.clone() ]) {
                Err(err) => return Err(err),
                Ok(val) => results.push(val)
            }
        }
        Ok(Var {
            ls_data: Some(results),
            num_data: None,
            int_data: None
        })
    } else {
        // Only floats. It's sin
        let (a, b) = vars[0].to_float().num_data.unwrap().to_cardinal();

        /*
        * How do we do this generally?
        * sin(x) = 0.5e^jx - 0.5e^-jx, so what if x is complex?
        *
        * sin(a + jb) = 0.5*e^j(a + jb) - 0.5*e^-j(a + jb)
        * = 0.5*(e^ja)*e^-b - 0.5*(e^-ja)*(e^b)
        * = 0.5*e^-b angle(a) - 0.5*e^b angle(-a)
        *
        * Note: It's only the imaginary part
        */

        let len1 = 0.5 * (-b).exp();
        let angle1 = a;
        let len2 = 0.5 * b.exp();
        let angle2 = -a;
        
        let val1 = FComplex::new_polar(len1, angle1);
        let val2 = FComplex::new_polar(len2, angle2);

        let var_data = val1 - val2;
        let (_, var_data_b) = var_data.to_cardinal();
        let var_data = FComplex::new_cardinal(var_data_b, 0.0); // Im(answer)

        Ok(Var {
            ls_data: None,
            num_data: Some(var_data),
            int_data: None
        })
    }
};

// Like sine, but e^x + e^x instead of -
pub const COS: fn(&Vec<Var>)->Result<Var, String> = |vars| {
    if vars.len() != 1 {
        Err(String::from("Expected one argument for sin."))
    } else if vars[0].ls_data.is_some() {
        let mut results = Vec::new();
        for var in vars[0].clone().ls_data.unwrap() {
            match SIN(&vec![ var.clone() ]) {
                Err(err) => return Err(err),
                Ok(val) => results.push(val)
            }
        }
        Ok(Var {
            ls_data: Some(results),
            num_data: None,
            int_data: None
        })
    } else {
        // Only floats. It's sin
        let (a, b) = vars[0].to_float().num_data.unwrap().to_cardinal();

        let len1 = 0.5 * (-b).exp();
        let angle1 = a;
        let len2 = 0.5 * b.exp();
        let angle2 = -a;
        
        let val1 = FComplex::new_polar(len1, angle1);
        let val2 = FComplex::new_polar(len2, angle2);

        let var_data = val1 + val2;
        let (var_data_a, _) = var_data.to_cardinal();
        let var_data = FComplex::new_cardinal(var_data_a, 0.0); // Re(answer)

        Ok(Var {
            ls_data: None,
            num_data: Some(var_data),
            int_data: None
        })
    }
};

pub const TAN: fn(&Vec<Var>)->Result<Var, String> = |vars| {
    if vars.len() != 1 {
        Err(String::from("Expected one argument for tan."))
    } else {
        match SIN(vars) {
            Err(err) => Err(err),
            Ok(sin_val) => {
                match COS(vars) {
                    Err(err) => Err(err),
                    Ok(cos_val) => Ok(sin_val / cos_val)
                }
            }
        }
    }
};

pub const LEN: fn(&Vec<Var>)->Result<Var, String> = |vars| {
    if vars.len() != 1 {
        Err(String::from("Expected on argument for len."))
    } else if vars[0].ls_data.is_some() {
        Ok(Var {
            ls_data: None,
            num_data: None,
            int_data: Some(IComplex::new_polar(vars[0].clone().ls_data.unwrap().len() as i64, 0))
        })
    } else {
        Ok(Var {
            ls_data: None,
            num_data: None,
            int_data: Some(IComplex::new_polar(1, 0))
        })
    }
};

pub const E: fn(&Vec<Var>)->Result<Var, String> = |vars| {
    if vars.len() != 0 {
        Err(String::from("e() does not take arguments."))
    } else {
        Ok(Var {
            ls_data: None,
            num_data: Some(FComplex::new_polar(consts::E, 0.0)),
            int_data: None
        })
    }
};

pub const PI: fn(&Vec<Var>)->Result<Var, String> = |vars| {
    if vars.len() != 0 {
        Err(String::from("pi() does not take arguments."))
    } else {
        Ok(Var {
            ls_data: None,
            num_data: Some(FComplex::new_polar(consts::PI, 0.0)),
            int_data: None
        })
    }
};

pub const ASIN: fn(&Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const ACOS: fn(&Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const ATAN: fn(&Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const D2R: fn(&Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const R2D: fn(&Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const LOG: fn(&Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const LN: fn(&Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const MOD: fn(&Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const FLOOR: fn(&Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const CEIL: fn(&Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const ABS: fn(&Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const IDX: fn(&Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const APP: fn(&Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const DEL: fn(&Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const SIGN: fn(&Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));
pub const COMP: fn(&Vec<Var>)->Result<Var, String> = |_| Err(String::from("Not implemented"));

