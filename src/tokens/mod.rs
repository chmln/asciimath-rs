mod operator;
mod token;

pub use self::{operator::Operator, token::Token};

pub type TokenList = Vec<Token>;
