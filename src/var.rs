/*
 * Author: Dylan Turner
 * Description:
 * - A variable type for use in the interpreter.
 * - Implements several functions between vars
 * - Real/Complex and Int/Float/List
 */

use bigdecimal::{BigDecimal, Zero, ToPrimitive, FromPrimitive};
use num::BigInt;
use std::{
    str::FromStr,
    ops::{
        Add, Sub, Mul, Div, BitXor
    }
};

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
            } else if self.lat_int_data.is_some() {
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

    pub fn to_float(&self) -> Self {
        let mut new_self = self.clone();

        if new_self.real_int_data.is_some() {
            new_self.real_num_data = Some(BigDecimal::from_str(
                new_self.real_int_data.unwrap().to_string().as_str()
            ).unwrap());
            new_self.real_int_data = None;
        } else if new_self.lat_int_data.is_some() {
            new_self.lat_num_data = Some(BigDecimal::from_str(
                new_self.lat_int_data.unwrap().to_string().as_str()
            ).unwrap());
            new_self.lat_int_data = None;
        } else if new_self.ls_data.is_some() {
            let mut new_ls = Vec::new();
            let ls = new_self.ls_data.unwrap();
            for var in ls {
                new_ls.push(var.to_float());
            }
            new_self.ls_data = Some(new_ls);
        }
        
        new_self
    }

    pub fn do_cmp(self, other: Self, op: &str) -> Self {
        match op {
            "=" => self.do_op(
                other,
                |a, b| if a == b {
                    BigDecimal::from_i8(-1).unwrap()
                } else {
                    BigDecimal::zero()
                }, |a, b| if a == b {
                    BigInt::from_i8(-1).unwrap()
                } else {
                    BigInt::zero()
                }
            ), "=/=" => self.do_op(
                other,
                |a, b| if a != b {
                    BigDecimal::from_i8(-1).unwrap()
                } else {
                    BigDecimal::zero()
                }, |a, b| if a != b {
                    BigInt::from_i8(-1).unwrap()
                } else {
                    BigInt::zero()
                }
            ), ">" => self.do_op(
                other,
                |a, b| if a > b {
                    BigDecimal::from_i8(-1).unwrap()
                } else {
                    BigDecimal::zero()
                }, |a, b| if a > b {
                    BigInt::from_i8(-1).unwrap()
                } else {
                    BigInt::zero()
                }
            ), "<" => self.do_op(
                other,
                |a, b| if a < b {
                    BigDecimal::from_i8(-1).unwrap()
                } else {
                    BigDecimal::zero()
                }, |a, b| if a < b {
                    BigInt::from_i8(-1).unwrap()
                } else {
                    BigInt::zero()
                }
            ), ">=" => self.do_op(
                other,
                |a, b| if a >= b {
                    BigDecimal::from_i8(-1).unwrap()
                } else {
                    BigDecimal::zero()
                }, |a, b| if a >= b {
                    BigInt::from_i8(-1).unwrap()
                } else {
                    BigInt::zero()
                }
            ), "<=" => self.do_op(
                other,
                |a, b| if a <= b {
                    BigDecimal::from_i8(-1).unwrap()
                } else {
                    BigDecimal::zero()
                }, |a, b| if a <= b {
                    BigInt::from_i8(-1).unwrap()
                } else {
                    BigInt::zero()
                }
            ), _ => {
                    println!("Here! {}", op);
                    Self::impossible()            
            }
        }
    }

    // Basically dec_op/int_op are +, -, etc, but this way I can reuse code
    pub fn do_op<
        DF: FnOnce(BigDecimal, BigDecimal) -> BigDecimal + Copy,
        IF: FnOnce(BigInt, BigInt) -> BigInt + Copy
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
                real_num_data: None,
                lat_num_data: None,
                real_int_data: None,
                lat_int_data: None        
            }
        } else if self.ls_data.is_some() {
            // One is list, so do the operation to with other to every item
            let mut cur = self.ls_data.clone().unwrap();
            for var in cur.iter_mut() {
                *var = var.clone().do_op(other.clone(), dec_op, int_op);
            }
            Var {
                ls_data: Some(cur),
                real_num_data: None,
                lat_num_data: None,
                real_int_data: None,
                lat_int_data: None        
            }
        } else if other.ls_data.is_some() {
            // One is list, so do the operation to with other to every item
            let mut cur = other.ls_data.clone().unwrap();
            for var in cur.iter_mut() {
                *var = var.clone().do_op(self.clone(), dec_op, int_op);
            }
            Var {
                ls_data: Some(cur),
                real_num_data: None,
                lat_num_data: None,
                real_int_data: None,
                lat_int_data: None        
            }
        } else {
            // Check if using floats as it overrides intedness
            if self.real_num_data.is_some() || self.lat_num_data.is_some()
                    || other.real_num_data.is_some() || other.lat_num_data.is_some() {
                let f_self = self.to_float();
                let f_other = other.to_float();

                let real = if f_self.real_num_data.is_some() && f_other.real_num_data.is_some() {
                    Some(dec_op(f_self.real_num_data.unwrap(), f_other.real_num_data.unwrap()))
                } else if f_self.real_num_data.is_some() {
                    f_self.real_num_data
                } else if f_other.real_num_data.is_some() {
                    f_other.real_num_data
                } else {
                    None
                };
                let lat = if f_self.lat_num_data.is_some() && f_other.lat_num_data.is_some() {
                    Some(dec_op(f_self.lat_num_data.unwrap(), f_other.lat_num_data.unwrap()))
                } else if f_self.lat_num_data.is_some() {
                    f_self.lat_num_data
                } else if f_other.lat_num_data.is_some() {
                    f_other.lat_num_data
                } else {
                    None
                };

                Var {
                    ls_data: None,
                    real_num_data: real,
                    lat_num_data: lat,
                    real_int_data: None,
                    lat_int_data: None
                }
            } else {
                // All ints
                let real = if self.real_int_data.is_some() && other.real_int_data.is_some() {
                    Some(int_op(self.real_int_data.unwrap(), other.real_int_data.unwrap()))
                } else if self.real_int_data.is_some() {
                    self.real_int_data
                } else if other.real_int_data.is_some() {
                    other.real_int_data
                } else {
                    None
                };
                let lat = if self.lat_int_data.is_some() && other.lat_int_data.is_some() {
                    Some(int_op(self.lat_int_data.unwrap(), other.lat_int_data.unwrap()))
                } else if self.lat_int_data.is_some() {
                    self.lat_int_data
                } else if other.lat_int_data.is_some() {
                    other.lat_int_data
                } else {
                    None
                };

                Var {
                    ls_data: None,
                    real_num_data: None,
                    lat_num_data: None,
                    real_int_data: real,
                    lat_int_data: lat
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
        self.do_op(
            other,
            |a, b| {
                let mut res = a.clone();
                for _ in 0..b.to_u32().expect("Non-integer exponent or too-big exponent") {
                    res *= a.clone();
                }
                res
            }, |a, b| a.pow(b.to_u32().expect("Power too large."))
        )
    }
}

