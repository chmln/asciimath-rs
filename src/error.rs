#[derive(PartialEq, Debug, thiserror::Error)]
pub enum Error {
    #[error("Expected more operands for operator \"{0}\"")]
    MissingOperands(crate::tokens::Operator), // operator
    #[error("Expected more parameters for function \"{0}\"")]
    NotEnoughFunctionParams(String), // fn name
    #[error(
        "Syntax error in function \"{0}()\". Usually this occurs because of \
         missing or extra commas."
    )]
    FunctionSyntaxError(String), // fn name
    #[error(
        "Function\"{0}()\" does not exist. If it is a custom function, make \
         sure you are passing it through the Scope."
    )]
    UnknownFunction(String), // fn name
    #[error(
        "Variable \"{0}\" is not defined. Make sure you are passing it \
         through the Scope."
    )]
    UnknownVariable(String), // var name
    #[error("Internal error. Please open an issue at https://github.com/chmln/asciimath-rs/issues")]
    CannotEvaluateToken(String), // invalid token on stack
    #[error("Invalid token: \"{0}\"")]
    InvalidToken(String),
    #[error("The expression is empty and there is nothing to evaluate")]
    EmptyExpression,
    #[error("Invalid operands given for operator:\"{0}\"")]
    InvalidOperands(crate::tokens::Operator),
    #[error(transparent)]
    Function(#[from] crate::constants::FunctionError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
