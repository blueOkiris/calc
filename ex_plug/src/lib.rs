#[derive(Clone, Copy)]
pub struct IComplex {
    pub len: i64,
    pub angle_deg: i64
}

#[derive(Clone, Copy)]
pub struct FComplex {
    pub len: f64,
    pub angle: f64
}

#[derive(Clone)]
pub struct Var {
    pub ls_data: Option<Vec<Var>>,
    pub num_data: Option<FComplex>,
    pub int_data: Option<IComplex>
}

#[no_mangle]
pub fn execute(vars: &Vec<Var>) -> Result<Var, String> {
    if vars.len() != 1 {
        Err(String::from("Expected one argument to function call."))
    } else {
        Ok(vars[0].clone())
    }
}

