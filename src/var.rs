/*
 * Author: Dylan Turner
 * Description:
 * - A variable type for use in the interpreter.
 * - Implements several functions between vars
 * - Real/Complex and Int/Float/List
 */

use std::ops::{
    Add, Sub, Mul, Div, BitXor
};
use crate::complex::{
    FComplex, IComplex
};

// Can be list, complex float, or complex int
#[derive(Clone, Debug)]
pub struct Var {
    pub ls_data: Option<Vec<Var>>,
    pub num_data: Option<FComplex>,
    pub int_data: Option<IComplex>
}

// Mainly the base functions 
impl Var {
    pub fn to_string(&self) -> String {
        if self.num_data.is_some() {
            self.num_data.unwrap().to_string()
        } else if self.int_data.is_some() {
            self.int_data.unwrap().to_string()
        } else if self.ls_data.is_some() {
            let mut repr = String::new();
            repr.push_str("[ ");
            for var in self.ls_data.clone().unwrap() {
                repr.push_str(var.to_string().as_str());
                repr.push(' ');
            }
            repr.push(']');
            repr
        } else {
            String::from("IMPOSSIBLE DATA ACHIEVED")
        }
    }

    pub fn impossible() -> Self {
        Self {
            ls_data: None,
            num_data: None,
            int_data: None
        }
    }

    pub fn to_lat(&self) -> Self {
        let mut new_self = self.clone();

        if new_self.ls_data.is_some() {
            let mut new_ls = Vec::new();
            let ls = new_self.ls_data.unwrap();
            for var in ls {
                new_ls.push(var.to_lat());
            }
            new_self.ls_data = Some(new_ls);
        } else if new_self.num_data.is_some() {
            let flipped = new_self.num_data.unwrap();
            let (real, _) = flipped.to_cardinal();
            new_self.num_data = Some(FComplex::new_cardinal(0.0, real));
        } else if new_self.int_data.is_some() {
            let flipped = new_self.int_data.unwrap();
            let (real, _) = flipped.to_cardinal();
            new_self.int_data = Some(IComplex::new_cardinal(0, real));
        }
        
        new_self
    }

    pub fn to_neg(&self) -> Self {
        let mut new_self = self.clone();

        if new_self.ls_data.is_some() {
            let mut new_ls = Vec::new();
            let ls = new_self.ls_data.unwrap();
            for var in ls {
                new_ls.push(var.to_neg());
            }
            new_self.ls_data = Some(new_ls);
        } else if new_self.num_data.is_some() {
            let data = new_self.num_data.unwrap();
            new_self.num_data = Some(FComplex::new_polar(-data.len, data.angle));
        } else if new_self.int_data.is_some() {
            let data = new_self.int_data.unwrap();
            new_self.int_data = Some(IComplex::new_polar(-data.len, data.angle_deg));
        }
        
        new_self
    }

    pub fn to_float(&self) -> Self {
        let mut new_self = self.clone();

        if new_self.ls_data.is_some() {
            let mut new_ls = Vec::new();
            let ls = new_self.ls_data.unwrap();
            for var in ls {
                new_ls.push(var.to_float());
            }
            new_self.ls_data = Some(new_ls);
        } else if new_self.int_data.is_some() {
            new_self.num_data = Some(new_self.int_data.unwrap().to_fcomplex());
        }
        
        new_self
    }

    pub fn do_cmp(self, other: Self, op: &str) -> Self {
        match op {
            "=" => self.do_op(
                other,
                |a, b| if a == b { FComplex::new_polar(-1.0, 0.0) } else { FComplex::zero() },
                |a, b| if a == b { IComplex::new_polar(-1, 0) } else { IComplex::zero() }
            ), "=/=" => self.do_op(
                other,
                |a, b| if a != b { FComplex::new_polar(-1.0, 0.0) } else { FComplex::zero() },
                |a, b| if a != b { IComplex::new_polar(-1, 0) } else { IComplex::zero() }
            ), ">" => self.do_op(
                other,
                |a, b| if a > b { FComplex::new_polar(-1.0, 0.0) } else { FComplex::zero() },
                |a, b| if a > b { IComplex::new_polar(-1, 0) } else { IComplex::zero() }
            ), "<" => self.do_op(
                other,
                |a, b| if a < b { FComplex::new_polar(-1.0, 0.0) } else { FComplex::zero() },
                |a, b| if a < b { IComplex::new_polar(-1, 0) } else { IComplex::zero() }
            ), ">=" => self.do_op(
                other,
                |a, b| if a >= b { FComplex::new_polar(-1.0, 0.0) } else { FComplex::zero() },
                |a, b| if a >= b { IComplex::new_polar(-1, 0) } else { IComplex::zero() }
            ), "<=" => self.do_op(
                other,
                |a, b| if a <= b { FComplex::new_polar(-1.0, 0.0) } else { FComplex::zero() },
                |a, b| if a <= b { IComplex::new_polar(-1, 0) } else { IComplex::zero() }
            ), _ => Self::impossible()
        }
    }

    // Basically dec_op/int_op are +, -, etc, but this way I can reuse code
    pub fn do_op<
        DF: FnOnce(FComplex, FComplex) -> FComplex + Copy,
        IF: FnOnce(IComplex, IComplex) -> IComplex + Copy
    >(self, other: Self, dec_op: DF, int_op: IF) -> Self {
        // Check for lists
        if self.ls_data.is_some() && other.ls_data.is_some() {
            // Both are lists, so do a matrix multiplication
            let mut new_ls = Vec::new();
            for var in self.ls_data.clone().unwrap() {
                let mut folded = var.clone();
                for other_var in other.ls_data.clone().unwrap() {
                    folded = folded.do_op(other_var.clone(), dec_op, int_op);
                }
                new_ls.push(folded);
            }
            Var {
                ls_data: Some(new_ls),
                num_data: None,
                int_data: None
            }
        } else if self.ls_data.is_some() {
            // One is list, so do the operation to with other to every item
            let mut cur = self.ls_data.clone().unwrap();
            for var in cur.iter_mut() {
                *var = var.clone().do_op(other.clone(), dec_op, int_op);
            }
            Var {
                ls_data: Some(cur),
                num_data: None,
                int_data: None
            }
        } else if other.ls_data.is_some() {
            // One is list, so do the operation to with other to every item
            let mut cur = other.ls_data.clone().unwrap();
            for var in cur.iter_mut() {
                *var = var.clone().do_op(self.clone(), dec_op, int_op);
            }
            Var {
                ls_data: Some(cur),
                num_data: None,
                int_data: None
            }
        } else {
            // Check if using floats as it overrides intedness
            if self.num_data.is_some() || self.num_data.is_some()
                    || other.num_data.is_some() || other.num_data.is_some() {
                let f_self = self.to_float();
                let f_other = other.to_float();

                let f_res = dec_op(f_self.num_data.unwrap(), f_other.num_data.unwrap());

                Var {
                    ls_data: None,
                    num_data: Some(f_res),
                    int_data: None
                }
            } else {
                // All ints
                let res = int_op(self.int_data.unwrap(), other.int_data.unwrap());
                Var {
                    ls_data: None,
                    num_data: None,
                    int_data: Some(res)
                }
            }
        }
    }
}

impl Add for Var {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        self.do_op(other, |a, b| a + b, |a, b| a + b)
    }
}

impl Sub for Var {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        self.do_op(other, |a, b| a - b, |a, b| a - b)
    }
}

impl Mul for Var {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        self.do_op(other, |a, b| a * b, |a, b| a * b) 
    }
}

impl Div for Var {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        self.do_op(other, |a, b| a / b, |a, b| a / b)
    }
}

// NOTE: Not xor but power
impl BitXor for Var {
    type Output = Self;
    fn bitxor(self, other: Self) -> Self {
        self.do_op(other, |a, b| a ^ b, |a, b| a ^ b)
    }
}

