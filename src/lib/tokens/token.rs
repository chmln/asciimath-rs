use tokens::{Number, Operator, Variable};

#[derive(Debug)]
pub enum Token {
    Operator(Operator),
    Number(Number),
    Variable(Variable),
    LeftParenthesis,
    RightParenthesis,
}
