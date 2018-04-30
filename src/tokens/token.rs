use tokens::{Function, Number, Operator, Variable};

#[derive(Debug, PartialEq)]
pub enum Token {
    Operator(Operator),
    Number(Number),
    Variable(Variable),
    LeftParenthesis,
    RightParenthesis,
    Comma,
    Function(Function),
}
