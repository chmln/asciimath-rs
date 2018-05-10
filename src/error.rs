use std::fmt;

#[derive(PartialEq, Debug)]
pub enum Error {
    MissingOperands(String),         // operator
    NotEnoughFunctionParams(String), // fn name
    FunctionSyntaxError(String),     // fn name
    UnknownFunction(String),         // fn name
    UnknownVariable(String),         // var name
    CannotEvaluateToken(String),     // invalid token on stack
    InvalidToken(String),
    EmptyExpression,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::MissingOperands(operator) => write!(
                f,
                "Expected more operands for operator \"{}\"",
                operator
            ),
            Error::NotEnoughFunctionParams(fn_name) => write!(
                f,
                "Expected more parameters for function \"{}\"",
                fn_name
            ),
            Error::FunctionSyntaxError(fn_name) => write!(
                f,
                "Syntax error in function \"{}()\". Usually this occurs \
                 because of missing or extra commas.",
                fn_name
            ),
            Error::UnknownFunction(fn_name) => write!(
                f,
                "Function\"{}()\" does not exist. If it is a custom function, \
                 make sure you are passing it through the Scope.",
                fn_name
            ),
            Error::UnknownVariable(var_name) => write!(
                f,
                "Variable \"{}\" is not defined. Make sure you are passing it \
                 through the Scope.",
                var_name
            ),
            Error::CannotEvaluateToken(token) => write!(
                f,
                "Token \"{}\" does not belong on the stack. Please open an \
                 issue with your expression at \
                 https://github.com/chmln/asciimath-rs/issues",
                token
            ),
            Error::InvalidToken(token) => {
                write!(f, "Invalid token: \"{}\"", token)
            },
            Error::EmptyExpression => write!(
                f,
                "The expression is empty and there is nothing to evaluate"
            ),
        }
    }
}
