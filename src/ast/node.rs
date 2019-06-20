use crate::{ast::Scope, tokens::Token};
use std::{collections::VecDeque, fmt};

pub type Args = VecDeque<Node>;

pub struct Node {
    pub token: Token,
    pub args: Option<Args>,
}

impl Node {
    pub(crate) fn new(token: Token, args: Option<Args>) -> Self {
        Self { token, args }
    }
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
