use crate::{ast::NumericLiteral, constants::CustomFn};
use std::{collections::HashMap, convert::Into};

pub enum Variable {
    Number(NumericLiteral),
    Function(CustomFn),
}

// Allows users to just pass in integers, not whatever type is used internally
macro_rules! num_into_var {
    ($($ty:ty)*) => {
        $(impl From<$ty> for Variable {
            fn from(val: $ty) -> Variable {
                Variable::Number(val as NumericLiteral)
            }
        })*
    }
}

num_into_var!(i8 i16 i32 i64 u8 u16 u32 u64 isize usize f32);

impl From<CustomFn> for Variable {
    fn from(val: CustomFn) -> Variable {
        Variable::Function(val)
    }
}

#[derive(Default)]
pub struct Scope {
    variables: HashMap<String, Variable>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            variables: HashMap::new(),
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Scope {
            variables: HashMap::with_capacity(cap),
        }
    }

    pub fn set_var<T: Into<Variable>>(&mut self, var_name: &str, value: T) {
        self.variables.insert(var_name.to_string(), value.into());
    }

    pub fn get_var(&self, var_name: &str) -> Option<&Variable> {
        self.variables.get(var_name)
    }
}
