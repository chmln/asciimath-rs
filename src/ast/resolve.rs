use crate::{
    ast::{Scope, Value, Variable},
    constants::{Func, CONSTANTS, FUNCTIONS},
    error::Error,
};

pub fn resolve_fn<'a>(name: &str, scope: &'a Scope) -> Result<&'a Func, Error> {
    FUNCTIONS.get(name).map_or_else(
        || match scope.get_var(name) {
            Some(Variable::Function(f)) => Ok(f),
            _ => Err(Error::UnknownFunction(name.to_string())),
        },
        |f| Ok(f),
    )
}

pub fn resolve_var(name: &str, scope: &Scope) -> Result<Value, Error> {
    CONSTANTS.get(name).map_or_else(
        || match scope.get_var(name) {
            Some(Variable::Number(x)) => Ok(Value::Num(*x)),
            Some(Variable::Boolean(x)) => Ok(Value::Bool(*x)),
            _ => Err(Error::UnknownVariable(name.to_string())),
        },
        |f| Ok(Value::Num(*f)),
    )
}
