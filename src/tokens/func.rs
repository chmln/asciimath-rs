#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
}

impl Function {
    pub fn new(name: String, _num_args: u8) -> Function {
        Function { name }
    }
}
