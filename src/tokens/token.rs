use crate::ast::NumericLiteral;
use crate::tokens::Operator;

#[derive(Debug, PartialEq)]
pub enum Token {
    Operator(Operator),
    Number(NumericLiteral),
    Variable(String),
    LeftParenthesis,
    RightParenthesis,
    Comma,
    Function(String),
}
