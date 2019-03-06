use crate::ast::Scope;
use std::{collections::VecDeque, fmt};
use crate::tokens::Token;

pub type Args = VecDeque<Node>;

pub struct Node {
    pub token: Token,
    pub args: Option<Args>,
}

pub struct Root<'a> {
    pub node: Node,
    pub scope: &'a Scope,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.token {
            Token::Variable(ref var) => write!(f, "{}", var),
            Token::Number(ref num) => write!(f, "{}", num),
            _ => write!(f, "({:?} {:?})", self.token, self.args),
        }
    }
}
