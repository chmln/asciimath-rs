#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
}

impl Function {
    pub fn new(name: String) -> Function {
        Function { name }
    }
}
