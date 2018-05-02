use std::{collections::VecDeque, fmt};
use tokens::Token;

pub struct Node {
    pub token: Token,
    pub args: Option<VecDeque<Node>>,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.token {
            Token::Variable(ref var) => write!(f, "{}", var.name),
            Token::Number(ref num) => write!(f, "{}", num.value),
            _ => write!(f, "({:?} {:?})", self.token, self.args),
        }
    }
}
