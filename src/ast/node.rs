use std::collections::VecDeque;
use tokens::Token;

pub struct Node {
    pub token: Token,
    pub args: Option<VecDeque<Node>>,
}
