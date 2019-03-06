use crate::ast::{NumericLiteral, Scope, Variable};
use crate::constants::{Func, CONSTANTS, FUNCTIONS};
use crate::error::Error;

pub fn resolve_fn<'a>(name: &str, scope: &'a Scope) -> Result<&'a Func, Error> {
    FUNCTIONS.get(name).map_or_else(
        || match scope.get_var(name) {
            Some(Variable::Function(f)) => Ok(f),
            _ => Err(Error::UnknownFunction(name.to_string())),
        },
        |f| Ok(f),
    )
}

pub fn resolve_var(name: &str, scope: &Scope) -> Result<NumericLiteral, Error> {
    CONSTANTS.get(name).map_or_else(
        || match scope.get_var(name) {
            Some(Variable::Number(n)) => Ok(*n),
            _ => Err(Error::UnknownVariable(name.to_string())),
        },
        |f| Ok(*f),
    )
}
