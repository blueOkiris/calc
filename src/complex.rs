/*
 * Author: Dylan Turner
 * Description: Custom number types for doing complex math
 */

use std::{
    f64::consts::PI,
    ops::{
        Add, Sub, Mul, Div, BitXor
    }
};

#[derive(Debug, Clone, Copy)]
pub struct FComplex {
    pub len: f64,
    pub angle: f64
}

impl FComplex {
    pub fn new_polar(len: f64, angle: f64) -> Self {
        Self {
            len, angle        
        }
    }

    pub fn new_cardinal(real: f64, lateral: f64) -> Self {
        let len = (real.powf(2.0) + lateral.powf(2.0)).sqrt();
        let angle = if real == 0.0 {
            PI
        } else if real > 0.0 {
            (lateral / real).atan()
        } else {
            (lateral / real).atan() + PI
        };

        Self::new_polar(len, angle)
    }

    pub fn to_cardinal(&self) -> (f64, f64) {
        let real = self.len * self.angle.cos();
        let lateral = self.len * self.angle.sin();
        (real, lateral)
    }
    
    pub fn to_string(&self) -> String {
        if self.angle != 0.0 {
            format!("{}∠{}", self.len, self.angle)    
        } else {
            format!("{}", self.len)
        }
    }
}

impl Add for FComplex {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let (r1, l1) = self.to_cardinal();
        let (r2, l2) = other.to_cardinal();
        Self::new_cardinal(r1 + r2, l1 + l2)
    }
}

impl Sub for FComplex {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let (r1, l1) = self.to_cardinal();
        let (r2, l2) = other.to_cardinal();
        Self::new_cardinal(r1 + r2, l1 + l2)
    }
}

impl Mul for FComplex {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::new_polar(self.len * other.len, self.angle + other.angle)
    }
}

impl Div for FComplex {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self::new_polar(self.len / other.len, self.angle - other.angle)
    }
}

// NOTE: Not xor but power. Also ignores lateral
impl BitXor for FComplex {
    type Output = Self;
    fn bitxor(self, other: Self) -> Self {
        /*
         * Polar form is r*e^jtheta, so what is (r*e^jtheta)^(r2*e^jtheta2)?
         *
         * Say we have complex z and complex w, z^w can be rewritten as e^(w * ln z)
         * Doing some math yields ln z = ln len(z) + j(2pi + angle(z))
         *
         * Which we can plug back in
         */

        // ln z = ln len(z) + j(2pi + angle(z))
        let ln_z = Self::new_cardinal(self.len.ln(), self.angle);
        
        // w * ln z
        let exp = other * ln_z;

        // e^(exp.a + jexp.b) = (e^exp.a, exp.b)
        let (exp_a, exp_b) = exp.to_cardinal();
        Self::new_polar(exp_a.exp(), exp_b)
    }
}

// Do it again with integer math
#[derive(Debug, Clone, Copy)]
pub struct IComplex {
    pub len: i64,
    pub angle_deg: i64
}

impl IComplex {
    pub fn new_polar(len: i64, angle_deg: i64) -> Self {
        Self {
            len, angle_deg
        }
    }

    pub fn new_cardinal(real: i64, lateral: i64) -> Self {
        let len = ((real.pow(2) + lateral.pow(2)) as f64).sqrt() as i64;
        let angle_deg = if real == 0 {
            180
        } else if real > 0 {
            (((lateral / real) as f64).atan() * 180.0 / PI) as i64
        } else {
            ((((lateral / real) as f64).atan() + PI) * 180.0 / PI) as i64
        };

        Self::new_polar(len, angle_deg)
    }

    pub fn to_cardinal(&self) -> (i64, i64) {
        let real = self.len * (((self.angle_deg as f64).cos() * 180.0 / PI) as i64);
        let lateral = self.len * (((self.angle_deg as f64).sin() * 180.0 / PI) as i64);
        (real, lateral)
    }

    pub fn to_fcomplex(&self) -> FComplex {
        FComplex::new_polar(self.len as f64, (self.angle_deg as f64) * PI / 180.0)
    }
     
    pub fn to_string(&self) -> String {
        if self.angle_deg != 0 {
            format!("{}∠{}°", self.len, self.angle_deg)    
        } else {
            format!("{}", self.len)
        }
    }
}

impl Add for IComplex {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let (r1, l1) = self.to_cardinal();
        let (r2, l2) = other.to_cardinal();
        Self::new_cardinal(r1 + r2, l1 + l2)
    }
}

impl Sub for IComplex {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let (r1, l1) = self.to_cardinal();
        let (r2, l2) = other.to_cardinal();
        Self::new_cardinal(r1 + r2, l1 + l2)
    }
}

impl Mul for IComplex {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::new_polar(self.len * other.len, self.angle_deg + other.angle_deg)
    }
}

impl Div for IComplex {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self::new_polar(self.len / other.len, self.angle_deg - other.angle_deg)
    }
}

// NOTE: Not xor but power. Also, ignores lateral
impl BitXor for IComplex {
    type Output = Self;
    fn bitxor(self, other: Self) -> Self {
        let result = self.to_fcomplex() ^ other.to_fcomplex();
        IComplex::new_polar(result.len as i64, (result.angle * 180.0 / PI) as i64)
    }
}


