#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub num_args: u8,
}

impl Function {
    pub fn new(name: String, num_args: u8) -> Function {
        Function { name, num_args }
    }
}
