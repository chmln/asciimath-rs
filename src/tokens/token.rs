use crate::{ast::NumericLiteral, tokens::Operator};

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
