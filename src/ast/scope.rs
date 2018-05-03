use ast::NumericLiteral;
use functions::CustomFunc;
use std::{collections::HashMap, convert::Into};

pub enum Variable {
    Number(NumericLiteral),
    Function(CustomFunc),
}

impl From<NumericLiteral> for Variable {
    fn from(val: NumericLiteral) -> Variable {
        Variable::Number(val)
    }
}

impl From<CustomFunc> for Variable {
    fn from(val: CustomFunc) -> Variable {
        Variable::Function(val)
    }
}

pub struct Scope {
    variables: HashMap<String, Variable>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            variables: HashMap::new(),
        }
    }

    fn set(&mut self, var_name: &str, value: Variable) {
        self.variables
            .insert(var_name.to_string(), value);
    }

    pub fn set_var<T>(&mut self, var_name: &str, value: T)
    where
        T: Into<NumericLiteral>,
    {
        self.set(var_name, Variable::Number(value.into()));
    }

    pub fn set_fn(&mut self, func_name: &str, function: CustomFunc) {
        self.set(func_name, Variable::Function(function));
    }

    pub fn get_var(&self, var_name: &str) -> Option<&Variable> {
        self.variables.get(var_name)
    }
}
