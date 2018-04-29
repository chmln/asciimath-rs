use tokens::{Number, Token, Variable};

pub enum Value {
    Number(Number),
    Token(Token),
    Variable(Variable),
}
